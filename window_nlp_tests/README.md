# granmo_model_nlp_classifier_rust
## flags, train mode

notes:
```
--model-type windowed (convolutional) vs. flat (bag of words, no context)

use --jsonl (flag without other input) for json(l)

# standard fast-test
  --epochs 7 \
  --clauses 10 \
  --max-features 100

# standard big maxy-test
  --epochs 32 \
  --clauses 256 \
  --max-features 6000

```
| Flag | Meaning | Default |
|---|---|---|
| `--mode train` | Required. Selects training. | — |
| `--train <PATH>` | Required. Training CSV file. | — |
| `--text-col <NAME>` | Column containing the text. | `text` |
| `--label-col <NAME>` | Column containing the category. | `label` |
| `--model-path <PATH>` | Where to save the trained model. If omitted, nothing is saved. | — |
| `--test <PATH>` | Separate test CSV. If omitted, the train file is split 80/20 automatically. | — |
| `--train-ratio <F>` | The automatic split ratio. | `0.8` |
| `--jsonl` | Input files are JSONL instead of CSV. | off |
| `--epochs <N>` | Training passes over the data. | `25` |
| `--clauses <N>` | Rules ("clauses") per class. More = bigger, slower model. | `80` |
| `--max-features <N>` | Vocabulary size cap. | `4000` |
| `--min-df <N>` | Word must appear in at least N documents to enter the vocabulary. | `2` |
| `--model-type <T>` | `windowed` = the new order-sensitive model. `flat` = the old-style bag-of-words model. | `windowed` |
| `--window-width <N>` | Windowed model only: how many consecutive words per window. | `3` |
| `--pooling <P>` | Windowed model only: `countfire` (count how many places a rule matched) or `anyfire` (only whether it matched at all). | `countfire` |
| `--vote-cap <N>` | Windowed model only: maximum votes one rule may contribute. | `3` |

+
specificity
threshold


## flags, predict mode

| Flag | Meaning |
|---|---|
| `--mode predict` | Required. |
| `--model-path <PATH>` | The saved model file to load. |
| `--text "<STRING>"` | The single text to classify. |


# cli call examples

#### .Train
```bash
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/archive_hsosd/train.csv \
  --text-col tweet \
  --label-col class \
  --model-type windowed \
  --model-path /home/oops/models/hsosd_model.json  \
  --epochs 10 \
  --clauses 40 \
  --max-features 1500
```

#### .Predict
```bash
cargo run --release -- \
  --mode predict \
  --model-path /home/ABC/models/hsosd_model.json \
  --text "Heavy earthquake tremors felt across the valley, buildings collapsed"
```

#### .Train
```bash
cargo run --release -- \
  --mode train \
  --train /home/ABC/datasets/disaster_tweets/train.csv \
  --text-col text \
  --label-col target \
  --model-type windowed \
  --model-path /home/ABC/tsetlin_models/disaster_model.json \
  --epochs 25 \
  --clauses 100 \
  --max-features 4000
```

#### .Predict
```bash
cargo run --release -- \
  --mode predict \
  --model-path /home/ABC/models/disaster_model.json \
  --text "Heavy earthquake tremors felt across the valley, buildings collapsed"
```

#### .Train
```bash
cargo run --release -- \
  --mode train \
  --train /home/ABC/datasets/imdb/IMDB\ Dataset.csv \
  --text-col review \
  --label-col sentiment \
  --model-type windowed \
  --model-path /home/ABC/tsetlin_models/IMDB_model.json \
  --epochs 20 \
  --clauses 150 \
  --max-features 5000
```
#### .Predict
```bash
cargo run --release -- \
  --mode predict \
  --model-path /home/ABC/tsetlin_models/IMDB_model.json \
  --text "Heavy earthquake tremors felt across the valley, buildings collapsed"
```

#### .Train
```bash
cargo run --release -- \
  --mode train \
  --train /home/ABC/datasets/spam/spam.csv \
  --text-col v2 \
  --label-col v1 \
  --model-type windowed \
  --model-path /home/ABC/tsetlin_models/IMDB_model.json \
  --epochs 20 \
  --clauses 60 \
  --max-features 3000
```

#### .Train
```bash
cargo run --release -- \
  --mode train \
  --train /home/ABC/datasets/fake_news/train.csv \
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/ABC/tsetlin_models/fake_news_model.json \
  --epochs 25 \
  --clauses 120 \
  --max-features 6000
```
#### .Predict
```bash
cargo run --release -- \
  --mode predict \
  --model-path /home/ABC/tsetlin_models/fake_news_model.json \
  --text "Heavy earthquake tremors felt across the valley, buildings collapsed"
```


### 1. `--max-features 6000` (Vocabulary Size / Literal Dimensionality)

#### Why 6,000 instead of 50,000 or 500?
* **Zipf’s Law in NLP**: In natural text, word frequency follows a power law. The top $3,000 - 6,000$ unigrams account for roughly **90%–95% of all semantic information** in news articles or reviews. Words beyond rank 6,000 are mostly typos, rare proper nouns, or noise that cause overfitting.
* **The Literal Multiplier ($2 \times D$)**:
  In a Tsetlin Machine, every feature $x_k$ produces two literals: the **positive literal** ($x_k$) and the **negated literal** ($\neg x_k$).
  Therefore, $6,000$ features translate to **$12,000$ Tsetlin Automata per clause**.
* **L3 CPU Cache Residency**:
  The total number of 32-bit state integers stored in RAM is:
  $$\text{Total States} = \text{Classes} \times \text{Clauses} \times 2 \times \text{MaxFeatures}$$
  For 2 classes, 120 clauses, and 6,000 features:
  $$2 \times 120 \times 12,000 = 2,880,000 \text{ integers} \approx \mathbf{11.5\text{ MB}}$$
  $11.5\text{ MB}$ fits entirely inside the **L3 cache** of modern x86/ARM processors (e.g., AMD Zen 3/4/5 or Intel Core), which allows bitwise clause evaluation to run at memory-bus speeds without spilling into slower main RAM.

---

### 2. `--clauses 120` (Model Capacity & Logic Expressivity)

#### Why 120 clauses per class?
* **Sub-pattern Decomposition**:
  A class concept (e.g., "Fake News" or "Disaster") cannot be captured by a single boolean formula. It is a disjunction of multiple sub-patterns:
  $$\text{Disaster} = (\text{earthquake} \land \text{shaking}) \lor (\text{wildfire} \land \neg\text{contained}) \lor (\text{flooding} \land \text{river}) \dots$$
  Each clause specializes in **one** specific sub-pattern or linguistic context.
* **Positive vs. Negative Voting Balance**:
  In standard TM architecture, clauses are divided into even ($+1$) and odd ($-1$) voters.
  With $120$ clauses per class:
  * **60 positive clauses** learn patterns that support the class.
  * **60 negative clauses** learn counter-evidence patterns (veto rules).
* **Statistical Ensemble Variance**:
  Granmo's research showed that for datasets of $5,000 - 25,000$ samples, $80 - 150$ clauses per class provide sufficient ensemble voting stability without redundant pattern duplication [1].

---

### 3. `--epochs 25` (State Space Traversal & Equilibrium)

#### Why 25 epochs?
* **Automata State Depth ($N = 100$, range $1 \dots 200$)**:
  A Tsetlin automaton starts at the neutral boundary ($N = 100$). To firmly transition a literal from `Exclude` (state $\le 100$) to `Include` (state $> 100$), it needs a sequence of consecutive reward steps.
* **Convergence Dynamics**:
  In NLP BoW vectors, words appear sparsely. With $10,000$ training documents:
  * In **Epochs 1–5**: The model eliminates frequent generic stop words via Type I inaction feedback.
  * In **Epochs 6–15**: Specific multi-word patterns crystallize into conjunctions.
  * In **Epochs 16–25**: Voting margins reach the threshold $T$, the stochastic update probability $(T - v) / 2T$ approaches 0, and the automata reach **Nash equilibrium** (stability).
* Training past 25–30 epochs on fixed-vocabulary text yields diminishing returns and plateaus.

---

### 4. How Internal Parameters Interlock ($T$, $s$, $N$)

Command-line arguments connect to mathematical hyperparameters inside the TM engine:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        Tsetlin Machine Tuning Formulas                 │
├──────────────────────────┬─────────────────────────────────────────────┤
│ Target Threshold (T)     │ T ≈ 0.5 × clauses  to  0.8 × clauses        │
│                          │ For clauses = 120  ──►  T = 50..80          │
├──────────────────────────┼─────────────────────────────────────────────┤
│ Specificity (s)          │ Controls clause length (literal count):     │
│                          │ s = 3.0..5.0  ──► Short clauses (2-4 words) │
│                          │ s = 6.0..10.0 ──► Long clauses (5-8 words)  │
├──────────────────────────┼─────────────────────────────────────────────┤
│ States per action (N)    │ N = 100 (Total 200 states)                  │
│                          │ Prevents noise from flipping logic decisions│
└──────────────────────────┴─────────────────────────────────────────────┘
```

---

### Summary Rule of Thumb for Your Own Experiments

When tuning for other datasets, you can apply this empirical scaling rule:

| Dataset Size & Type | `--max-features` | `--clauses` | `--epochs` | Specificity ($s$) |
| :--- | :--- | :--- | :--- | :--- |
| **Small / Short** (e.g. 5k SMS/Tweets) | `3000` | `60 - 80` | `20` | `3.5` (keeps rules broad) |
| **Medium** (e.g. 10k–20k News/Articles) | `5000 - 6000` | `100 - 150` | `25` | `5.0` (balanced) |
| **Large / Long** (e.g. 50k IMDb Reviews) | `8000 - 10000` | `200 - 300` | `25 - 30` | `7.0` (filters narrative noise) |



In this two-stage neuro-symbolic pipeline, a **3-way split (Train / Validation / Test)** plays a distinct role compared to standard deep learning or linear models. 

Because we perform **unsupervised semantic discovery (Stage 1)** before **supervised logic learning (Stage 2)**, strict separation is required to prevent **data leakage** and **automata overfitting**.

---

### 1. The 3-Way Data Lifecycle Flow

```
                           Raw Labeled Corpus (CSV / JSONL)
                                         │
                   ┌─────────────────────┼─────────────────────┐
                   │                     │                     │
             Train Set (70%)       Val/Dev Set (15%)     Test Set (15%)
                   │                     │                     │
   ┌───────────────┴───────────────┐     │                     │
   │ 1. Build Vocab & Compute IDF  │     │                     │
   │ 2. Fit K-Means Centroids      │     │                     │
   │ 3. Pre-train NTM on Clusters  │     │                     │
   │ 4. Extract Topic Keywords     │     │                     │
   └───────────────┬───────────────┘     │                     │
                   │                     │                     │
                   ▼                     ▼                     ▼
             [Train BoW]            [Val BoW]              [Test BoW]
                   │                     │                     │
                   │ <─── Predict K-Means & Enrich ────────────┤
                   ▼                     ▼                     ▼
             [Enriched Train]       [Enriched Val]         [Enriched Test]
                   │                     │                     │
                   ▼                     │                     │
         Train Vanilla TM (Epoch e)      │                     │
                   │                     │                     │
                   └──────────► Evaluate Val Accuracy          │
                                (Checkpoint Best States)       │
                                         │                     │
                                         ▼                     │
                              Restore Best Model ─────────────►│
                                                               ▼
                                                      Final Unbiased Report
```

---

### 2. The Role of Each Split in this Architecture

#### A. Training Set ($70\% - 80\%$)
Used for **all parameter estimation**:
1. **Vocabulary & IDF tables**: Determines word frequency rankings and IDF weights.
2. **K-Means Clustering**: Determines semantic cluster centroids.
3. **Stage 1 (NTM)**: Automata learn cluster-defining conjunctions to extract semantic descriptors.
4. **Stage 2 (Vanilla TM)**: Automata update literal inclusion/exclusion states to learn classification rules.

#### B. Validation / Dev Set ($10\% - 15\%$)
Used for **model selection and hyperparameter optimization**:
* **Epoch-by-Epoch Checkpointing**: At the end of each training epoch, the validation set is evaluated without updating automata states. If the validation Macro F1 score reaches a new high, a snapshot of the `ta_states` is saved.
* **Hyperparameter Tuning**: Used to tune specificity $s$, threshold $T$, clause counts, and cluster count $K$ without touching the test set.

#### C. Test Set ($10\% - 15\%$)
Used **strictly once** at the very end:
* It is evaluated solely on the best checkpointed model to provide an unbiased generalization benchmark.

---

### 3. Anti-Leakage Rules

When running NLP benchmarks with Tsetlin Machines, follow these three rules:

1. **Vocabulary Isolation**: Never fit the `Vocabulary` or compute `idf` on the combined corpus. Test and validation documents must be vectorized using the **training set’s vocabulary**. Out-of-vocabulary (OOV) words in the test set are automatically ignored.
2. **Centroid Isolation**: Never fit `KMeans` on test or validation samples. To find the cluster ID of a test document, use `kmeans.predict(&test_tfidf)` using the **training centroids**.
3. **Keyword Dictionary Freezing**: Once Stage 1 finishes on the training set, the `SemanticEnricher` keyword lookup table is frozen.

---

### 4. Code Implementation: 3-Way Split with Validation Checkpointing

Here is how the dataset splitting and validation-driven checkpointing are implemented in Rust:

#### Step 1: 3-Way Dataset Splitter
```rust
impl Dataset {
    /// Splits the dataset into Train, Validation, and Test subsets (e.g., 0.70, 0.15, 0.15).
    pub fn split_3way(
        &self,
        train_ratio: f64,
        val_ratio: f64,
        rng: &mut FastRng,
    ) -> Result<(Dataset, Dataset, Dataset), PipelineError> {
        if train_ratio + val_ratio >= 1.0 || train_ratio <= 0.0 || val_ratio <= 0.0 {
            return Err(PipelineError::InvalidConfiguration(
                "train_ratio + val_ratio must be strictly less than 1.0".to_string(),
            ));
        }

        let mut shuffled = self.records.clone();
        for i in (1..shuffled.len()).rev() {
            let j = rng.gen_range(0, i + 1)?;
            shuffled.swap(i, j);
        }

        let train_end = ((shuffled.len() as f64) * train_ratio).round() as usize;
        let val_end = train_end + ((shuffled.len() as f64) * val_ratio).round() as usize;

        let train_set = Dataset {
            records: shuffled[0..train_end].to_vec(),
            label_to_id: self.label_to_id.clone(),
            id_to_label: self.id_to_label.clone(),
        };

        let val_set = Dataset {
            records: shuffled[train_end..val_end].to_vec(),
            label_to_id: self.label_to_id.clone(),
            id_to_label: self.id_to_label.clone(),
        };

        let test_set = Dataset {
            records: shuffled[val_end..].to_vec(),
            label_to_id: self.label_to_id.clone(),
            id_to_label: self.id_to_label.clone(),
        };

        Ok((train_set, val_set, test_set))
    }
}
```

#### Step 2: Epoch-by-Epoch Validation Loop & Checkpointing
```rust
/// Helper function to compute accuracy on any dataset split without modifying TM state.
fn evaluate_split(
    dataset: &Dataset,
    tm: &VanillaTM,
    vocab: &Vocabulary,
    kmeans: &KMeans,
    enricher: &SemanticEnricher,
) -> Result<f64, PipelineError> {
    let mut correct = 0usize;
    for record in &dataset.records {
        let label_id = match dataset.label_to_id.get(&record.label) {
            Some(&id) => id,
            None => continue,
        };
        let raw_bow = vocab.text_to_bow(&record.text);
        let tfidf = vocab.text_to_tfidf(&record.text);
        let cluster_id = kmeans.predict(&tfidf)?;
        let enriched = enricher.enrich(&raw_bow, cluster_id);

        let pred = tm.predict(&enriched)?;
        if pred == label_id {
            correct += 1;
        }
    }
    Ok(correct as f64 / dataset.records.len() as f64)
}

// Inside the Stage 2 training loop:
let mut best_val_acc = 0.0;
let mut best_tm_model = tm.clone();

for epoch in 1..=config.epochs {
    // 1. Train on training samples
    for record in &train_set.records {
        let label_id = train_set.label_to_id[&record.label];
        let raw_bow = vocab.text_to_bow(&record.text);
        let tfidf = vocab.text_to_tfidf(&record.text);
        let cluster_id = kmeans.predict(&tfidf)?;
        let enriched = enricher.enrich(&raw_bow, cluster_id);

        tm.train_step(&enriched, label_id, &mut rng)?;
    }

    // 2. Validate on validation samples
    let val_acc = evaluate_split(&val_set, &tm, &vocab, &kmeans, &enricher)?;
    
    // 3. Checkpoint the best model
    if val_acc > best_val_acc {
        best_val_acc = val_acc;
        best_tm_model = tm.clone();
        println!("  Epoch {epoch:2} | Val Accuracy: {:.2}% (★ New Best)", val_acc * 100.0);
    }
}

// 4. Final test evaluation uses best_tm_model
```

---

### 5. Summary

| Phase | Dataset Used | Modifies Model Weights/States? | Purpose |
| :--- | :--- | :---: | :--- |
| **Vocab / IDF** | **Train** | Yes | Build token dictionary and inverse document frequencies. |
| **Stage 1 (NTM)** | **Train** | Yes | Induce semantic cluster keywords. |
| **Stage 2 (TM)** | **Train** | Yes | Learn propositional classification rules. |
| **Validation** | **Val / Dev** | **No** | Monitor epoch progress, trigger early stopping, and save best model snapshot. |
| **Benchmark** | **Test** | **No** | Generate the final confusion matrix and classification report. |


# Notes

### What "Specificity" ($s$) Does in a Tsetlin Machine

In Ole-Christoffer Granmo’s Tsetlin Machine formulation, **Specificity ($s \ge 1.0$) is the learning rate and regularization control knob** for logical clauses. It directly controls whether the model learns **short, general rules** or **long, highly specific rules**.

During training (Type I feedback):
* **Inclusion probability (reinforcing matching features):**
  $$\mathbb{P}(\text{Include}) = \frac{s - 1}{s} = 1 - \frac{1}{s}$$
* **Forgetting / Erasing probability (clearing non-matching features):**
  $$\mathbb{P}(\text{Exclude / Forget}) = \frac{1}{s}$$

| Value of $s$ | What it means | Clause Behavior | Best For |
| :--- | :--- | :--- | :--- |
| **Low $s$ (e.g., $1.5 - 2.5$)** | High erasure rate ($1/2 = 50\%$). Clauses shed literals quickly. | Clauses remain very **short and general** (e.g., `[w+0] hate`). | Noisy data, short texts, small vocabularies where you want coarse rules. |
| **Moderate $s$ (e.g., $3.0 - 4.0$)** | Balanced reinforcement ($67\% - 75\%$) and forgetting ($25\% - 33\%$). | Clauses learn moderate conjunctions (e.g., `[w+0] not ∧ [w+1] good`). | Standard NLP text classification. |
| **High $s$ (e.g., $5.0 - 10.0$)** | Low erasure rate ($1/10 = 10\%$). Clauses accumulate features rapidly. | Clauses become **long, strict, and complex** (many `AND` conditions). | Dense tabular data, exact pattern matching, or low-noise synthetic tasks. |
