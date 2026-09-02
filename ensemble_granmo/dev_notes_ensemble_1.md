
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset raw  --engine seq-freq-hybrid \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/test.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
```


$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset raw  --engine seq-freq-hybrid \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/test.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset raw --engine seq-freq-hybrid --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 2 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/test.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: SeqFreqHybrid, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 2, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: seq-freq-hybrid)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:00:46
------------------------------------------------------------
  Accuracy (@ V > 0): 70.00%
  Best-F1 Threshold:  V > -8
  Precision:          0.6907
  Recall:             0.7965
  F1-Score:           0.7398
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3186        1769        
Actual Pos (1)    1009        3950        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 38/200  always 37/200 (37 vacuous, 0 specialized)  p25 1.7%  median 14.2%  p75 27.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 21  (37 clauses vacuous)
  vacuous vote offset: -3  (17 positive-polarity, 20 negative-polarity vacuous)
============================================================

misprediction log: appended 2974 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
saved model artifact to /home/oops/models/test.gmb



```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset raw  --engine seq-freq-hybrid \
  --clauses 200 --vote-threshold 80 --states 150 \
  --specificity 3.0 --vocab-size 4000 --ngram-len 5 \
  --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/test.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
```
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset raw  --engine seq-freq-hybrid \
  --clauses 200 --vote-threshold 80 --states 150 \
  --specificity 3.0 --vocab-size 4000 --ngram-len 5 \
  --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/test.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset raw --engine seq-freq-hybrid --clauses 200 --vote-threshold 80 --states 150 --specificity 3.0 --vocab-size 4000 --ngram-len 5 --epochs 8 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/test.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: SeqFreqHybrid, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 4000, n_clauses: 200, vote_threshold: 80, states_per_action: 150, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 8, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: seq-freq-hybrid)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:05:32
------------------------------------------------------------
  Accuracy (@ V > 0): 58.44%
  Best-F1 Threshold:  V > -33
  Precision:          0.6989
  Recall:             0.8296
  F1-Score:           0.7587
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3183        1772        
Actual Pos (1)    845         4114        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 50/400  always 56/400 (56 vacuous, 0 specialized)  p25 5.1%  median 9.8%  p75 20.6%
  includes/clause: min 0  p25 1  median 1  p75 1  max 113  (56 clauses vacuous)
  vacuous vote offset: -26  (15 positive-polarity, 41 negative-polarity vacuous)
============================================================

misprediction log: appended 4120 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
saved model artifact to /home/oops/models/test.gmb

note: vary preprocess:
p1

```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p1  --engine byte-bag \
  --clauses 200 --vote-threshold 80 --states 150 \
  --specificity 3.0 --vocab-size 4000 --ngram-len 5 \
  --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/test.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
```
