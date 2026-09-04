Best scores for: (note seed)

e.g.
(C=400, T=100, M=10k, s=4.2)

--clauses 400) — "C" Clause Bank Capacity
--vote-threshold 100) — "T" The Margin / Confidence Ceiling
--vocab-size 10000) — "M" Feature Space / Vocabulary Size
--specificity 4.2) — "s" The Pattern Strictness Knob


# Accuracy: 85.49%
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out

# Precision: 0.8606
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out
  
# Recall: 0.8887
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 11 --seed 128 --workers auto \
  --train-percent 80 \

  
# F1-Score: 0.8556
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 11 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt


Goals:

1. Finding different minima/equilibria/solution-geometries

2. Best Confusion-Matrix 1-Metric (across seeds)

Comparing seeds:
While given set of parameters can perform well across random-seeds,
from seed to seed 'narrow' type or 'wide' type may have better metrics (precision, f1, recall, accuracy, FP, etc.)


---
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  vs
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \

# Best Narrow

42
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:13:55
------------------------------------------------------------
  Accuracy (@ V > 0): 84.68%
  Best-F1 Threshold:  V > 0
  Precision:          0.8497
  Recall:             0.8427
  F1-Score:           0.8462
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4216        739         
Actual Pos (1)    780         4179        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 21.3%  median 24.0%  p75 26.8%
  includes/clause: min 1  p25 33  median 38  p75 44  max 62  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1519 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

```

128
```bash
$  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
  
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:30
------------------------------------------------------------
  Accuracy (@ V > 0): 85.49%
  Best-F1 Threshold:  V > -1
  Precision:          0.8606
  Recall:             0.8485
  F1-Score:           0.8545
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4244        685         
Actual Pos (1)    755         4230        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 23.4%  median 25.7%  p75 28.1%
  includes/clause: min 1  p25 32  median 37  p75 42  max 58  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1439 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

```

---

# Best Wide
- Type Wide

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
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:30:00
------------------------------------------------------------
  Accuracy (@ V > 0): 84.95%
  Best-F1 Threshold:  V > 0
  Precision:          0.8408
  Recall:             0.8625
  F1-Score:           0.8515
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4145        810         
Actual Pos (1)    682         4277        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.3%  median 20.5%  p75 22.7%
  includes/clause: min 1  p25 55  median 63  p75 71  max 97  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1492 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
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


- lowerer  to  --specificity 3.7 DID help -seed 128
--vote-threshold 110

- raising to --states 87                    did NOT help -seed 42, 128, 248
- lower to --clauses 294                    did NOT help -seed 42, 128, 248
- lowerer  to  --specificity 3.6            did NOT help -seed 42, 128, 248
- lowering --vote-threshold                 did NOT help (seed 42)
- lowering vocab size lowered accuracy, raised recall (seed 42)
- changing epoch to 11 or 13                did NOT help -seed 42,


*vs.*

## Wide Type
- larger vocabulary
- more clauses
- larger n-gram length

- raising voting threshold to 105 DID help
- raising ngram-len to 6          DID help
- lowering states --states 84     DID help

- increasing to --vote-threshold 110             did not help (seeds 42 128 248)
- increasing specificity to --specificity 4.3    did not help (seeds 42 128 248)
  - increase to '--ngram-len 7'                  did NOT help (seeds 42 128 248)
- increase to '--vocab-size 10500'               did NOT help (seeds 42 128 248)
- increase to '--vocab-size 12000 --ngram-len 7' did NOT help (seeds 42 128 248)




--- 
*Tests:*
