Best scores for:
  Accuracy:
  Precision:
  Recall:
  F1-Score:

Goals:

1. Finding different minima/equilibria/solution-geometries

2. Best Confusion-Matrix 1-Metric (across seeds)

Comparing seeds:
While given set of parameters can perform well across random-seeds,
from seed to seed 'narrow' type or 'wide' type may have better metrics (precision, f1, recall, accuracy, FP, etc.)


---



# Best Narrow

42
```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:18:25
------------------------------------------------------------
  Accuracy (@ V > 0): 84.63%
  Best-F1 Threshold:  V > 0
  Precision:          0.8394
  Recall:             0.8566
  F1-Score:           0.8479
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4142        813         
Actual Pos (1)    711         4248        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 20.6%  median 23.3%  p75 25.9%
  includes/clause: min 1  p25 34  median 40  p75 46  max 59  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1524 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
```

128
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:16:01
------------------------------------------------------------
  Accuracy (@ V > 0): 85.17%
  Best-F1 Threshold:  V > -1
  Precision:          0.8607
  Recall:             0.8381
  F1-Score:           0.8493
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4253        676         
Actual Pos (1)    807         4178        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 22.7%  median 24.9%  p75 27.6%
  includes/clause: min 1  p25 33  median 38  p75 42  max 65  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1470 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
```

---

# Best Wide
- Type Wide

```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 100, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:20:06
------------------------------------------------------------
  Accuracy (@ V > 0): 84.57%
  Best-F1 Threshold:  V > 0
  Precision:          0.8372
  Recall:             0.8584
  F1-Score:           0.8477
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4127        828         
Actual Pos (1)    702         4257        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.2%  median 20.6%  p75 23.0%
  includes/clause: min 1  p25 54  median 63  p75 71  max 97  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1530 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb

```

128:
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:08:33
------------------------------------------------------------
  Accuracy (@ V > 0): 85.14%
  Best-F1 Threshold:  V > -1
  Precision:          0.8597
  Recall:             0.8383
  F1-Score:           0.8489
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4247        682         
Actual Pos (1)    806         4179        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.9%  median 22.1%  p75 24.5%
  includes/clause: min 1  p25 52  median 59  p75 65  max 92  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1473 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb

```

248
```bash
$ cargo run --release -- --mode train   --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl   --preset p0 --engine byte-bag   --clauses 300 --vote-threshold 75 --states 85   --specificity 3.8 --vocab-size 8000 --ngram-len 5   --max-scan 4096 --epochs 12 --seed 248 --workers auto   --train-percent 80   --model-out /home/oops/models/imdb_p0_c300_e12.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:06:11
------------------------------------------------------------
  Accuracy (@ V > 0): 84.15%
  Best-F1 Threshold:  V > 1
  Precision:          0.8202
  Recall:             0.8862
  F1-Score:           0.8519
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4017        959         
Actual Pos (1)    562         4376        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 21.4%  median 24.2%  p75 27.0%
  includes/clause: min 1  p25 32  median 39  p75 45  max 64  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1571 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
```


...

# Best F1
```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
   Compiling para_byte_granmo v0.1.0 (/home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo)
    Finished `release` profile [optimized] target(s) in 10.81s
     Running `target/release/para_byte_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 100, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 02:45:14
------------------------------------------------------------
  Accuracy (@ V > 0): 84.41%
  Best-F1 Threshold:  V > 1
  Precision:          0.8443
  Recall:             0.8615
  F1-Score:           0.8528
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4167        788         
Actual Pos (1)    687         4272        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.6%  median 20.9%  p75 23.3%
  includes/clause: min 1  p25 44  median 52  p75 60  max 82  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1546 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
```

# Best Precision 
- Precision:         
```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.9 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.9 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.9, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:15:47
------------------------------------------------------------
  Accuracy (@ V > 0): 84.40%
  Best-F1 Threshold:  V > 1
  Precision:          0.8493
  Recall:             0.8443
  F1-Score:           0.8468
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4212        743         
Actual Pos (1)    772         4187        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 20.0%  median 22.8%  p75 25.1%
  includes/clause: min 1  p25 35  median 41  p75 47  max 65  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
```

...
# Best Recall
-   Recall:             0.8742

```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 110 \
  --specificity 3.7 --vocab-size 7500 --ngram-len 5 \
  --max-scan 8192 --epochs 9 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_scan4k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 190 --vote-threshold 50 --states 110 --specificity 3.7 --vocab-size 7500 --ngram-len 5 --max-scan 8192 --epochs 9 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_scan4k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 7500, n_clauses: 190, vote_threshold: 50, states_per_action: 110, specificity: 3.7, max_scan_bytes: 8192, guarded_include: false, fire_guard_streak_limit: 0, epochs: 9, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:09:21
------------------------------------------------------------
  Accuracy (@ V > 0): 83.16%
  Best-F1 Threshold:  V > -5
  Precision:          0.7983
  Recall:             0.8742
  F1-Score:           0.8345
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3860        1095        
Actual Pos (1)    624         4335        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/190  always 6/190 (6 vacuous, 0 specialized)  p25 17.3%  median 22.6%  p75 25.3%
  includes/clause: min 0  p25 1  median 38  p75 45  max 66  (6 clauses vacuous)
  vacuous vote offset: +0  (3 positive-polarity, 3 negative-polarity vacuous)
============================================================

misprediction log: appended 1670 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
saved model artifact to /home/oops/models/imdb_p0_scan4k.gmb
```

...

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 95 \
  --specificity 4.5 --vocab-size 7000 --ngram-len 5 \
  --max-scan 4096 --epochs 10 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c200_e11.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e12.txt
   Compiling ensemble_granmo v0.1.0 (/home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo)
    Finished `release` profile [optimized] target(s) in 6.07s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 190 --vote-threshold 50 --states 95 --specificity 4.5 --vocab-size 7000 --ngram-len 5 --max-scan 4096 --epochs 10 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c200_e11.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 7000, n_clauses: 190, vote_threshold: 50, states_per_action: 95, specificity: 4.5, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 10, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:07:49
------------------------------------------------------------
  Accuracy (@ V > 0): 83.57%
  Best-F1 Threshold:  V > -1
  Precision:          0.8135
  Recall:             0.8605
  F1-Score:           0.8363
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3977        978         
Actual Pos (1)    692         4267        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/190  always 0/190 (0 vacuous, 0 specialized)  p25 17.9%  median 19.7%  p75 22.0%
  includes/clause: min 1  p25 39  median 45  p75 50  max 62  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1629 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c200_e11.gmb
...


```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 200 --vote-threshold 50 --states 100 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 11 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c200_e11.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e11.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 200 --vote-threshold 50 --states 100 --specificity 4.0 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 11 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c200_e11.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e11.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 200, vote_threshold: 50, states_per_action: 100, specificity: 4.0, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 11, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:09:20
------------------------------------------------------------
  Accuracy (@ V > 0): 83.45%
  Best-F1 Threshold:  V > -3
  Precision:          0.8252
  Recall:             0.8558
  F1-Score:           0.8402
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4056        899         
Actual Pos (1)    715         4244        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/200  always 1/200 (1 vacuous, 0 specialized)  p25 20.1%  median 22.5%  p75 24.5%
  includes/clause: min 0  p25 36  median 43  p75 47  max 64  (1 clauses vacuous)
  vacuous vote offset: -1  (0 positive-polarity, 1 negative-polarity vacuous)
============================================================

misprediction log: appended 1641 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e11.txt
saved model artifact to /home/oops/models/imdb_p0_c200_e11.gmb
```



...
# Best Recall
-   Recall:             0.8742

```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 110 \
  --specificity 3.7 --vocab-size 7500 --ngram-len 5 \
  --max-scan 8192 --epochs 9 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_scan4k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 190 --vote-threshold 50 --states 110 --specificity 3.7 --vocab-size 7500 --ngram-len 5 --max-scan 8192 --epochs 9 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_scan4k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 7500, n_clauses: 190, vote_threshold: 50, states_per_action: 110, specificity: 3.7, max_scan_bytes: 8192, guarded_include: false, fire_guard_streak_limit: 0, epochs: 9, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:09:21
------------------------------------------------------------
  Accuracy (@ V > 0): 83.16%
  Best-F1 Threshold:  V > -5
  Precision:          0.7983
  Recall:             0.8742
  F1-Score:           0.8345
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3860        1095        
Actual Pos (1)    624         4335        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/190  always 6/190 (6 vacuous, 0 specialized)  p25 17.3%  median 22.6%  p75 25.3%
  includes/clause: min 0  p25 1  median 38  p75 45  max 66  (6 clauses vacuous)
  vacuous vote offset: +0  (3 positive-polarity, 3 negative-polarity vacuous)
============================================================

misprediction log: appended 1670 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
saved model artifact to /home/oops/models/imdb_p0_scan4k.gmb
```
...


$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 200 --vote-threshold 50 --states 100 \
  --specificity 3.5 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_scan4k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 200 --vote-threshold 50 --states 100 --specificity 3.5 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 8 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_scan4k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 200, vote_threshold: 50, states_per_action: 100, specificity: 3.5, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 8, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 02:45:14
------------------------------------------------------------
  Accuracy (@ V > 0): 83.01%
  Best-F1 Threshold:  V > -2
  Precision:          0.8120
  Recall:             0.8518
  F1-Score:           0.8314
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3977        978         
Actual Pos (1)    735         4224        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/200  always 8/200 (8 vacuous, 0 specialized)  p25 13.7%  median 22.6%  p75 26.8%
  includes/clause: min 0  p25 1  median 35  p75 41  max 63  (8 clauses vacuous)
  vacuous vote offset: +6  (7 positive-polarity, 1 negative-polarity vacuous)
============================================================

misprediction log: appended 1684 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
saved model artifact to /home/oops/models/imdb_p0_scan4k.gmb


---

# Strategy Notes: 
A. What to vary looking for 'better' metrics (which metric)
B. Looking for different equilibira to try to recombine later:
- Q Seed
- Q Vote Threshold
- Q Epochs
- Q ngram-len
- Q --max-scan 4096 


## Narrow Type
- smaller vocabulary
- fewer clauses
- (clauses can't be under 300? some ratio with something else?)

- lowering --vote-threshold did NOT help (seed 42)
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
```

*vs.*

## Wide Type
-  Higher F1
- F1->85.28
- larger vocabulary
- more clauses

- raising voting threshold to 105 DID help
- raising ngram-len to 6 DID help

```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
```




--- 
*Tests:*

ngram-len 4
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 4 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
  
/////
specificity 3.8 
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt

  ...

  seed 24
  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 24 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt



cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 290 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt


...

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt





  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt


specificity 4.3
  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.3 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt




--vote-threshold
     cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 74 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt


-vote-threshold 100
  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt



  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt



--states 85
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 86 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt


  --states 85
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 84 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt



  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
