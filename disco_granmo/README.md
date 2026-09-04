##### Documentation on dataset analysis with suggests by Claude Sonnet

# Dataset Analysis and Model Training — Workflow Guide

This tool has two phases: analyzing the dataset for problems, then training the model.

## Phase 1: Dataset Analysis

**Goal:** Find rows that are likely mislabeled or problematic before they corrupt training.

### Step 1: Run a k-fold sweep to generate prediction records

```bash
cargo run --release -- --mode train \
  --data /abs/path/data.jsonl \
  --folds 5 --split-seed 7 \
  --score-train-side \
  --records-out /abs/path/predictions.tsv \
  --preset p0 --engine byte-bag \
  --clauses 200 --states 50 --epochs 8 --seed 42
```

(No `--model-out` — the 5 models trained here aren't kept.)

This splits the dataset into 5 folds and trains 5 models, each tested on the fold it did not train on. Every row gets tested once by a model that never saw it. `--score-train-side` records how each model scores rows that it *did* train on. All predictions are written to `predictions.tsv`.

### Step 2: Generate the suspect-row report

```bash
cargo run --release -- --mode row-audit \
  --records-in /abs/path/predictions.tsv \
  --audit-top 100
```

This reads the recorded predictions and ranks rows by: 
1. (If that row was) Never predicted correctly on any *test* fold
2. (If that row was) Predicted incorrectly even when the model *trained* on that row
3. How strongly the model's vote disagrees with the label

### Step 3: Interpret the report

A row appearing/ranked near the top of the report means that the model consistently disagreed with the row's class-label. This can mean:
- the label is wrong (fix or remove)
- the text is genuinely ambiguous (probably remove)
- or the pattern is rare but the label is correct (maybe more examples needed for such a case to be learned)

The report is a prioritized list to check by hand — not an automatic verdict. Some tests can be automated, but decisions about datasets need to be made by domain-expert people.

### Step 4: Review flagged-rows and clean the dataset

Look up the flagged line numbers in the source file, read the text and label, and decide case by case. Save a cleaned copy of the dataset with the confirmed bad rows removed (recommended: keep a record of what was removed (including the tests used, data, and who modified the data, ideally with the parameters used so the test can be repeated exactly), and some copy of the original dataset).

## Phase 2: Model Training

**Goal:** Using the cleaned dataset configure model-parameters and finally to Train and publish/export a model to keep and use.

### Step 1: Batch comparison to pick a configuration

```bash
cargo run --release -- --mode batch \
  --data /abs/path/cleaned_data.jsonl \
  --seed 42 --train-percent 80
```

This terminal command runs the preset × engine comparison matrix on one split, so you can see which configuration performs best on this dataset.

### Step 2: Train and save the final model

```bash
cargo run --release -- --mode train \
  --data /abs/path/cleaned_data.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 200 --states 50 --epochs 8 \
  --seed 42 \
  --model-out /abs/path/final_model.gmb
```

`--folds` defaults to 1 (not multiple folds). There is no implemented cross-validation for model training (yet).


### Step 3: Check seed sensitivity manually

Run Step 2 again a few times with different `--seed` values and compare accuracy/F1 across runs (there is no implemented cross-validation for model training (yet)). This tells you how much of your result depends on training randomness rather than the data or configuration. There is no built-in command that automates this averaging — it's a manual comparison across runs. 

Note: There can be very different ~equilibria of parameters for best-fit. So if you have time, test significantly different starting values and tweak up and down to check for improvements. 

E.g. here are two different 'best model' parameter sets for the same IMDB dataset:
```bash
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
```
  *vs*
```bash
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
```


### Tips: 
- If you think there may be a bug in your input command (if it simply exits), leave out the "--release" in the input to enable more verbose error and debut messages.
