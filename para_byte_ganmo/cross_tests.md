# Grain of Salt: Claude Haiku suggests...

## Suggested settings by dataset size and exploration stage

The principle: **start cheap, learn the shape, confirm once.** Use learning curves and vacuity/includes diagnostics to avoid the 7-hour full runs until you know what you're looking for.

---

## Stage 1: First look at a new dataset (5–10 min per run)

**Purpose:** Does the dataset work at all? What's the class balance? Are there label errors?

```bash
# Quick sanity check on a new dataset
cargo run --release -- --mode train \
  --data /path/to/new_dataset.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 50 --states 50 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /tmp/model_sanity.gmb \
  --log-out /tmp/mispred_sanity.txt
```

**What to read:**
- Accuracy at V>0: ballpark (50% = random; <40% = class imbalance or label errors).
- Confusion matrix shape: if FN >> FP, your positive class is rare or hard; if FP >> FN, your labels may be flipped.
- **Misprediction log**: open `/tmp/mispred_sanity.txt` and spot-check 10 high-|V| errors. If they look like *label errors* (text clearly contradicts label), you have a cleaning problem; if they look *hard but correct*, the model is working.
- Vacuity report: if >50% clauses are vacuous, the dataset is too small or too easy (N=50 is already shallow; this signals the data can't sustain specialization).

---

## Stage 2: Learning curve (understand the dataset, 20–30 min total)

**Purpose:** Does more training data help? Do more epochs help? What's the asymptotic accuracy?

Run three sizes at 4 epochs each, hold eval fixed:

```bash
# Small (first 10k docs of training)
cargo run --release -- --mode train \
  --data /path/to/new_dataset.jsonl \
  --preset p0 \
  --clauses 200 --vote-threshold 80 --states 100 \
  --specificity 3.0 --vocab-size 2000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --train-percent 80 --test-cap 5000

# Medium (first 30k docs)
cargo run --release -- --mode train \
  --data /path/to/new_dataset.jsonl \
  --preset p0 \
  --clauses 200 --vote-threshold 80 --states 100 \
  --specificity 3.0 --vocab-size 2000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --train-percent 80 --test-cap 5000

# Full (all docs)
cargo run --release -- --mode train \
  --data /path/to/new_dataset.jsonl \
  --preset p0 \
  --clauses 200 --vote-threshold 80 --states 100 \
  --specificity 3.0 --vocab-size 2000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80
```

**What to read:**
- Plot F1 vs. train-doc count. If curve is climbing steeply at full size, merging more data is high-ROI. If it's flat from 30k onward, you've hit the ceiling on this task (settings tuning is the lever, not data).
- Vacuity trend: does it shrink as training size grows? If it stays high at 30k, raise `--states` to 50 and retest cheap (the N=200 bimodal-specialization problem may apply here too).
- Includes median: if it jumps from 0–5 range to 100+ range between 10k and 30k, that's a sign the smaller dataset is underfitting; at full size you have the budget to specialize.

---

## Stage 3: Preset sweep (understand preprocessing, 15–20 min)

**Purpose:** Which preprocessing helps on this dataset?

```bash
# Run p0, p1, p2 at once (use your existing --mode batch, but subset)
cargo run --release -- --mode batch \
  --data /path/to/new_dataset.jsonl \
  --clauses 200 --vote-threshold 80 --states 100 \
  --specificity 3.0 --vocab-size 2000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --train-percent 80 --test-cap 5000
  # This will run p0, p1, p2 (hardcoded in handle_batch), byte-conv and byte-bag each
```

**What to read:**
- Best F1 per preset (p0 likely wins; p2 leet-fold helps on teen slang datasets; p1/p3 rarely help).
- Engine comparison within each preset: if conv >> bag or vice versa, the dataset has strong positional signal (conv) or bag-of-shingles signal (bag).
- Vacuity/includes per preset: sometimes a preprocessing choice unlocks specialization (e.g. lowercase helps on all-caps hate posts by reducing surface sparsity).

---

## Stage 4: Hyperparameter sweep for production (once you know the shape, 30–45 min)

**Purpose:** What settings squeeze the last 0.5–2% F1 out of this dataset?

Use `--mode batch-guard` with a frozen full eval set and a modest train subsample (balances speed vs. signal):

```bash
# Depth sweep (N matters most for specialization)
cargo run --release -- --mode batch-guard \
  --data /path/to/new_dataset.jsonl \
  --preset p0 \
  --clauses 300 --vote-threshold 100 --specificity 3.0 \
  --vocab-size 3000 --ngram-len 5 \
  --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --guard-limits 0 \
  # This runs one row (guard-off, byte-bag, p0) at full size

# Then rerun with different --states values:
# --states 75, --states 150, --states 200
# (Record all three outputs, ignore --guard-limits since it's 0)
```

**What to read:**
- Which N gave best F1? If it's 75, you were right to suspect N=200 was overkill. If it's 200, you need deeper (try 300, but expect diminishing returns).
- Vacuity vs. N: plot the vacuous-count trend. Where does it drop below ~5%? That's the "depth is sufficient" point.
- Includes distribution: once vacuity is controlled, you're looking at the median and p75 to decide if the specialization depth is healthy (median >50 is good; median <10 says "more epochs needed").

---

## Stage 5: Final blessed model (confirmation, 7–15 min)

**Purpose:** Train once, log everything, save the artifact.

Once you've found good settings via the above (e.g. N=100, vocab=3000, epochs=8, best preset = p2):

```bash
cargo run --release -- --mode train \
  --data /path/to/new_dataset.jsonl \
  --preset p2 \
  --clauses 300 --vote-threshold 100 --states 100 \
  --specificity 3.0 --vocab-size 3000 --ngram-len 5 \
  --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /models/blessed_$(date +%s).gmb \
  --log-out /logs/mispred_blessed.txt
```

Copy the full command to a run-log file with date and dataset hash. This is your reproducible baseline.

---

## Suggested preset settings across dataset sizes

| Dataset Size | Clauses | T | N | Vocab | Epochs | Expected time | Use case |
|---|---|---|---|---|---|---|---|
| < 5k | 100 | 50 | 50 | 500 | 2 | ~30s | Exploration only |
| 5k–20k | 150 | 60 | 75 | 1000 | 3–4 | ~3 min | Learning curves |
| 20k–100k | 200–300 | 80–100 | 100 | 2000–3000 | 6–8 | ~8–12 min quick, 30–45 min full | Hyperparameter sweep |
| > 100k | 400–600 | 120–160 | 150–200 | 3000–4000 | 10–12 | ~45 min–2 hrs | Production |

**Rationale:**
- **Clauses scale slowly with data.** Doubling dataset size → ~20% more clauses, not 2x. Larger pools just need a bit more capacity.
- **N (depth) scales with epoch budget.** At fixed epochs, the limit is how many times an automaton can cross the N boundary. Small datasets with high-N are vacuous; huge datasets can sustain deep automata.
- **Vocab size is an experiment knob.** 2000–4000 is the working range; 1000 is too tight for large corpora, 5000+ has diminishing returns (most shingles past 4k are typos/noise). Rare exception: domain-specific work (legal text, code) might benefit from 5000–8000.

---

## Added CLI flag recommendation: `--train-cap` (to make learning curves easy)

Since you'll be running learning curves frequently, add `--train-cap` to `--mode train` (not just batch-guard):

```bash
cargo run --release -- --mode train \
  --data /abs/path/dataset.jsonl \
  --preset p0 \
  --clauses 200 --vote-threshold 80 --states 100 \
  --specificity 3.0 --vocab-size 2000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --train-percent 80 \
  --train-cap 10000 \
  --test-cap 5000
```

**One-line edit** in `CliArgs::to_run_config()`: after the train/test split but before preprocessing, add:

```rust
if args.train_cap > 0 && train_documents.len() > args.train_cap {
    train_documents.truncate(args.train_cap);
}
```

(Field and parse arm already exist from earlier edits; this just applies the cap in `run_single_experiment` before `preprocess_documents`.)

---

## Cross-dataset validation protocol (once you have 2+ datasets)

Once you're confident in settings A on dataset 1, test generalization before merging:

```bash
# Train on dataset 1, test on dataset 2
cargo run --release -- --mode train \
  --data /path/to/dataset_1.jsonl \
  --preset p0 \
  --clauses 200 --vote-threshold 80 --states 100 \
  --specificity 3.0 --vocab-size 2000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /tmp/model_ds1.gmb

# Then predict on dataset 2 (requires a --mode predict batch extension —
# or manually: for each line in dataset_2.jsonl, run --mode predict,
# collect scores, compute confusion matrix by hand or script)
```

Record the cross-dataset F1 matrix:

```
        Test on DS1  Test on DS2  Test on DS3
Train DS1    97%        72%         85%
Train DS2    68%        96%         89%
Train DS3    81%        88%         95%
Merged      94%        91%         92%
```

If all off-diagonal scores are >80% of on-diagonal, merging is safe. If any are <70%, that dataset conflicts — keep it separate or debug the label mapping.

---

## Quick checklist for each new dataset

```
☐ Stage 1: sanity check (5 min)
  ☐ Accuracy > 50%? (No = label error, wrong task, or class imbalance)
  ☐ Misprediction log OK? (spot-check 5 records)
  ☐ Vacuity < 50%?

☐ Stage 2: learning curve (25 min)
  ☐ F1 at 10k, 30k, full train sizes
  ☐ Vacuity trend: shrinking or flat?
  ☐ Includes median: is specialization happening?

☐ Stage 3: preset sweep (15 min, optional if stage 1 was p0-only)
  ☐ Any preset beats p0?
  ☐ Conv vs. bag — who wins?

☐ Stage 4: N sweep (30 min)
  ☐ Best N for this dataset?
  ☐ Vacuity disappears by N=?

☐ Stage 5: final train (15 min)
  ☐ Save artifact + command line
  ☐ Record to experiment log

☐ Cross-dataset matrix (if 2+ datasets): TBD later
```

---

## One caveat: the settings I've given are *not* optimized for your current dataset

Your 97% on the 80k cyberbully set was trained with N=200, clauses=600, vocab=4000, epochs=12. The settings above (N=100, clauses=200, vocab=2000, epochs=6–8) are **conservative starting points** for an *unknown* dataset. Once you know a new dataset is tractable, you can afford to spend 30 min on a full-N sweep to match or beat 97%.


...
