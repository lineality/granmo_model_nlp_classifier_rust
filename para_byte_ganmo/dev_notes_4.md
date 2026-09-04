(C=400, T=100, M=10k, s=4.2)

--clauses 400) — "C" Clause Bank Capacity
--vote-threshold 100) — "T" The Margin / Confidence Ceiling
--vocab-size 10000) — "M" Feature Space / Vocabulary Size
--specificity 4.2) — "s" The Pattern Strictness Knob




  //////////////////
 # windowed test

```bash
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --text-col text \
  --label-col label \
  --jsonl \
  --model-type flat \
  --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json \
  --epochs 2 \
  --clauses 200 \
  --threshold 90 \
  --specificity 5 \
  --max-features 4000
```
////

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 95 \
  --specificity 4.5 --vocab-size 7000 --ngram-len 5 \
  --max-scan 4096 --epochs 10 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c200_e11.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e12.txt
  

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 200 --vote-threshold 50 --states 100 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 11 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c200_e11.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e11.txt



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


...

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 200 --vote-threshold 50 --states 70 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 8 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_n70_e8.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_n70_e8.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/para_byte_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 200 --vote-threshold 50 --states 70 --specificity 4.0 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 8 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_n70_e8.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_n70_e8.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 200, vote_threshold: 50, states_per_action: 70, specificity: 4.0, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 8, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 02:45:14
------------------------------------------------------------
  Accuracy (@ V > 0): 83.12%
  Best-F1 Threshold:  V > -1
  Precision:          0.8399
  Recall:             0.8187
  F1-Score:           0.8292
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4181        774         
Actual Pos (1)    899         4060        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/200  always 0/200 (0 vacuous, 0 specialized)  p25 20.8%  median 23.6%  p75 26.4%
  includes/clause: min 7  p25 31  median 36  p75 42  max 58  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1673 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_n70_e8.txt
saved model artifact to /home/oops/models/imdb_p0_n70_e8.gmb

...

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 200 --vote-threshold 50 --states 100 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 4 \
  --max-scan 4096 --epochs 10 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_ngram4_e10.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_ngram4_e10.txt
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/para_byte_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 200 --vote-threshold 50 --states 100 --specificity 4.0 --vocab-size 8000 --ngram-len 4 --max-scan 4096 --epochs 10 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_ngram4_e10.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_ngram4_e10.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 8000, n_clauses: 200, vote_threshold: 50, states_per_action: 100, specificity: 4.0, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 10, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 02:45:14
------------------------------------------------------------
  Accuracy (@ V > 0): 82.09%
  Best-F1 Threshold:  V > 2
  Precision:          0.8312
  Recall:             0.8391
  F1-Score:           0.8351
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4110        845         
Actual Pos (1)    798         4161        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/200  always 3/200 (3 vacuous, 0 specialized)  p25 20.5%  median 23.0%  p75 25.3%
  includes/clause: min 0  p25 36  median 41  p75 46  max 67  (3 clauses vacuous)
  vacuous vote offset: +3  (3 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1776 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_ngram4_e10.txt
saved model artifact to /home/oops/models/imdb_p0_ngram4_e10.gmb


...


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




cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p1 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 110 \
  --specificity 3.5 --vocab-size 7000 --ngram-len 5 \
  --max-scan 8192 --epochs 9 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_scan4k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt


...
P1

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p1 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 110 \
  --specificity 3.5 --vocab-size 7000 --ngram-len 5 \
  --max-scan 8192 --epochs 9 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_scan4k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p1 --engine byte-bag --clauses 190 --vote-threshold 50 --states 110 --specificity 3.5 --vocab-size 7000 --ngram-len 5 --max-scan 8192 --epochs 9 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_scan4k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 7 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 7000, n_clauses: 190, vote_threshold: 50, states_per_action: 110, specificity: 3.5, max_scan_bytes: 8192, guarded_include: false, fire_guard_streak_limit: 0, epochs: 9, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:05:37
------------------------------------------------------------
  Accuracy (@ V > 0): 82.87%
  Best-F1 Threshold:  V > -2
  Precision:          0.8158
  Recall:             0.8439
  F1-Score:           0.8296
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4010        945         
Actual Pos (1)    774         4185        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/190  always 9/190 (9 vacuous, 0 specialized)  p25 12.3%  median 23.2%  p75 25.6%
  includes/clause: min 0  p25 1  median 33  p75 41  max 53  (9 clauses vacuous)
  vacuous vote offset: +3  (6 positive-polarity, 3 negative-polarity vacuous)
============================================================

misprediction log: appended 1698 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt
saved model artifact to /home/oops/models/imdb_p0_scan4k.gmb

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

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 110 \
  --specificity 3.7 --vocab-size 7500 --ngram-len 5 \
  --max-scan 8192 --epochs 9 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_scan4k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_scan4k.txt

...
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
...



cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 190 --vote-threshold 50 --states 95 \
  --specificity 4.5 --vocab-size 7000 --ngram-len 5 \
  --max-scan 4096 --epochs 10 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c200_e11.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c200_e12.txt







--guarded



cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt


  cargo run --release -- --mode train \
    --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
    --preset p0 --engine byte-bag \
    --clauses 500 --vote-threshold 125 --states 85 \
    --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
    --max-scan 4096 --epochs 12 --seed 42 --workers auto \
    --train-percent 80 \
    --model-out /home/oops/models/imdb_p0_c500_m8k.gmb \
    --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c500_m8k.txt



    cargo run --release -- --mode train \
      --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
      --preset p0 --engine seq-freq-hybrid \
      --clauses 150 --vote-threshold 75 --states 85 \
      --patch 5 --stride 2 \
      --bag-vocab-size 8000 --ngram-len 5 \
      --specificity 4.0 --max-scan 4096 --epochs 12 --seed 42 --workers auto \
      --train-percent 80 \
      --model-out /home/oops/models/imdb_hybrid_c300.gmb \
      --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_hybrid_c300.txt


--vocab-size 12000?



cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.5 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_s45.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_s45.txt


cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 500 --vote-threshold 125 --states 85 \
  --specificity 4.3 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c500_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c500_m10k.txt


oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
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
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$


oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
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
============================================================

misprediction log: appended 1547 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ ^C
oops@fedora:~$ cd code/granmo_model_nlp_classifier_rust/ensemble_granmo/
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 4 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
  
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 4 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 100, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:20:40
------------------------------------------------------------
  Accuracy (@ V > 0): 83.56%
  Best-F1 Threshold:  V > 1
  Precision:          0.8232
  Recall:             0.8711
  F1-Score:           0.8465
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4027        928         
Actual Pos (1)    639         4320        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.8%  median 21.1%  p75 23.4%
  includes/clause: min 1  p25 42  median 52  p75 63  max 106  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1630 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
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
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ ^C
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 290 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 290 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 290, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }


- Accuracy (@ V > 0): 84.52%
```bash
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 4.0 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 4.0, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:13:34
------------------------------------------------------------
  Accuracy (@ V > 0): 84.52%
  Best-F1 Threshold:  V > 0
  Precision:          0.8397
  Recall:             0.8534
  F1-Score:           0.8465
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4147        808         
Actual Pos (1)    727         4232        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 19.5%  median 22.0%  p75 24.7%
  includes/clause: min 1  p25 37  median 42  p75 48  max 67  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1535 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
```


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
Training Time Duration (h:m:s): 00:19:34
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


$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 74 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 74 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 74, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:18:12
------------------------------------------------------------
  Accuracy (@ V > 0): 84.37%
  Best-F1 Threshold:  V > 0
  Precision:          0.8447
  Recall:             0.8423
  F1-Score:           0.8435
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4187        768         
Actual Pos (1)    782         4177        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 21.1%  median 23.3%  p75 25.5%
  includes/clause: min 1  p25 33  median 39  p75 45  max 69  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1550 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

$ cargo run --release -- --mode train \
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
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.3 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 100, states_per_action: 85, specificity: 4.3, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:08
------------------------------------------------------------
  Accuracy (@ V > 0): 84.86%
  Best-F1 Threshold:  V > 0
  Precision:          0.8331
  Recall:             0.8719
  F1-Score:           0.8521
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4089        866         
Actual Pos (1)    635         4324        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.0%  median 20.1%  p75 22.5%
  includes/clause: min 3  p25 56  median 64  p75 73  max 104  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1501 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb


$ cargo run --release -- --mode train \
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
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 100 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 100, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:20:37
------------------------------------------------------------
  Accuracy (@ V > 0): 84.92%
  Best-F1 Threshold:  V > -1
  Precision:          0.8499
  Recall:             0.8459
  F1-Score:           0.8479
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4184        745         
Actual Pos (1)    768         4217        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.7%  median 22.0%  p75 24.6%
  includes/clause: min 1  p25 42  median 50  p75 56  max 73  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1495 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb

$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 4.0 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 24 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 4.0 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 24 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 4.0, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 24, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:18:11
------------------------------------------------------------
  Accuracy (@ V > 0): 83.65%
  Best-F1 Threshold:  V > -1
  Precision:          0.8308
  Recall:             0.8439
  F1-Score:           0.8373
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4119        850         
Actual Pos (1)    772         4173        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 2/300 (2 vacuous, 0 specialized)  p25 21.8%  median 24.1%  p75 26.7%
  includes/clause: min 0  p25 35  median 39  p75 43  max 58  (2 clauses vacuous)
  vacuous vote offset: +2  (2 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1621 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

$ cargo run --release -- --mode train   --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl   --preset p0 --engine byte-bag   --clauses 300 --vote-threshold 75 --states 85   --specificity 3.8 --vocab-size 8000 --ngram-len 5   --max-scan 4096 --epochs 12 --seed 64 --workers auto   --train-percent 80   --model-out /home/oops/models/imdb_p0_c300_e12.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 64 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 64, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:09:38
------------------------------------------------------------
  Accuracy (@ V > 0): 84.11%
  Best-F1 Threshold:  V > -4
  Precision:          0.8360
  Recall:             0.8389
  F1-Score:           0.8375
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4138        816         
Actual Pos (1)    799         4161        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 22.2%  median 25.1%  p75 27.7%
  includes/clause: min 1  p25 32  median 38  p75 43  max 58  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1575 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb


seed 128

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

$   cargo run --release -- --mode train   --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl   --preset p0 --engine byte-bag   --clauses 400 --vote-threshold 100 --states 85   --specificity 4.2 --vocab-size 10000 --ngram-len 5   --max-scan 4096 --epochs 12 --seed 64 --workers auto   --train-percent 80   --model-out /home/oops/models/imdb_p0_c400_m10k.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 100 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 64 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 100, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 64, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:11:22
------------------------------------------------------------
  Accuracy (@ V > 0): 84.19%
  Best-F1 Threshold:  V > -2
  Precision:          0.8485
  Recall:             0.8230
  F1-Score:           0.8355
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4225        729         
Actual Pos (1)    878         4082        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 1/400 (1 vacuous, 0 specialized)  p25 19.9%  median 22.6%  p75 25.3%
  includes/clause: min 0  p25 43  median 49  p75 55  max 77  (1 clauses vacuous)
  vacuous vote offset: +1  (1 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1567 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 290 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 290 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 290, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:06:45
------------------------------------------------------------
  Accuracy (@ V > 0): 84.42%
  Best-F1 Threshold:  V > 0
  Precision:          0.8363
  Recall:             0.8560
  F1-Score:           0.8460
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4124        831         
Actual Pos (1)    714         4245        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/290  always 0/290 (0 vacuous, 0 specialized)  p25 19.8%  median 22.7%  p75 25.2%
  includes/clause: min 1  p25 34  median 41  p75 47  max 66  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1545 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb



$ cargo run --release -- --mode train \
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


$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 11 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 11 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 11, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:23:20
------------------------------------------------------------
  Accuracy (@ V > 0): 84.24%
  Best-F1 Threshold:  V > 1
  Precision:          0.8250
  Recall:             0.8887
  F1-Score:           0.8556
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3989        940         
Actual Pos (1)    555         4430        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 1/300 (1 vacuous, 0 specialized)  p25 22.0%  median 24.7%  p75 27.6%
  includes/clause: min 0  p25 32  median 38  p75 43  max 64  (1 clauses vacuous)
  vacuous vote offset: +1  (1 positive-polarity, 0 negative-polarity vacuous)


comparing raise vs. lower states in wide model


$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 86 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 86 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 86, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:26:08
------------------------------------------------------------
  Accuracy (@ V > 0): 84.43%
  Best-F1 Threshold:  V > 0
  Precision:          0.8214
  Recall:             0.8800
  F1-Score:           0.8497
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4006        949         
Actual Pos (1)    595         4364        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.3%  median 20.5%  p75 22.9%
  includes/clause: min 2  p25 54  median 63  p75 72  max 99  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1544 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb


$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 84 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 84 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 84, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:26:42
------------------------------------------------------------
  Accuracy (@ V > 0): 84.59%
  Best-F1 Threshold:  V > 0
  Precision:          0.8257
  Recall:             0.8770
  F1-Score:           0.8506
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4037        918         
Actual Pos (1)    610         4349        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.4%  median 21.0%  p75 23.1%
  includes/clause: min 1  p25 53  median 61  p75 71  max 106  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1528 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb



not helped change epoch quantity:
$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 13 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 13 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 13, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:26:28
------------------------------------------------------------
  Accuracy (@ V > 0): 84.11%
  Best-F1 Threshold:  V > 1
  Precision:          0.8516
  Recall:             0.8381
  F1-Score:           0.8448
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4201        728         
Actual Pos (1)    807         4178        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 1/300 (1 vacuous, 0 specialized)  p25 22.2%  median 24.3%  p75 26.6%
  includes/clause: min 0  p25 34  median 38  p75 43  max 56  (1 clauses vacuous)
  vacuous vote offset: +1  (1 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1575 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb

  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.8 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 11 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 85 --specificity 3.8 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 11 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 85, specificity: 3.8, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 11, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:23:20
------------------------------------------------------------
  Accuracy (@ V > 0): 84.24%
  Best-F1 Threshold:  V > 1
  Precision:          0.8250
  Recall:             0.8887
  F1-Score:           0.8556
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3989        940         
Actual Pos (1)    555         4430        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 1/300 (1 vacuous, 0 specialized)  p25 22.0%  median 24.7%  p75 27.6%
  includes/clause: min 0  p25 32  median 38  p75 43  max 64  (1 clauses vacuous)
  vacuous vote offset: +1  (1 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1562 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb


top narrow
$ cargo run --release -- --mode train \
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
$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 12000 --ngram-len 7 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 12000 --ngram-len 7 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 12000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:33:58
------------------------------------------------------------
  Accuracy (@ V > 0): 83.97%
  Best-F1 Threshold:  V > -2
  Precision:          0.8213
  Recall:             0.8719
  F1-Score:           0.8459
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4014        941         
Actual Pos (1)    635         4324        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 1/400 (1 vacuous, 0 specialized)  p25 17.4%  median 20.3%  p75 22.3%
  includes/clause: min 0  p25 75  median 83  p75 93  max 136  (1 clauses vacuous)
  vacuous vote offset: -1  (0 positive-polarity, 1 negative-polarity vacuous)
============================================================

misprediction log: appended 1589 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb


cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt



  oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
    --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
    --preset p0 --engine byte-bag \
    --clauses 400 --vote-threshold 105 --states 85 \
    --specificity 4.2 --vocab-size 12000 --ngram-len 7 \
    --max-scan 4096 --epochs 12 --seed 128 --workers auto \
    --train-percent 80 \
    --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
    --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
      Finished `release` profile [optimized] target(s) in 0.01s
       Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 12000 --ngram-len 7 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
  loaded 49570 labeled documents
  resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 12000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }
  
  ============================================================
                 Classification Evaluation Report             
  ============================================================
    Run Preset:        p0           (Engine: byte-bag)
    Train/Test Split:  39656/9914 samples
  Training Time Duration (h:m:s): 00:33:54
  ------------------------------------------------------------
    Accuracy (@ V > 0): 81.41%
    Best-F1 Threshold:  V > -9
    Precision:          0.8287
    Recall:             0.8588
    F1-Score:           0.8435
  ------------------------------------------------------------
  Confusion Matrix (at optimal threshold):
                    Pred Neg (0)Pred Pos (1)
  Actual Neg (0)    4044        885         
  Actual Pos (1)    704         4281        
  ------------------------------------------------------------
  Clause Dynamics:
    fire-rate over 9914 test docs: never 0/400  always 7/400 (7 vacuous, 0 specialized)  p25 18.2%  median 20.8%  p75 23.0%
    includes/clause: min 0  p25 68  median 79  p75 88  max 120  (7 clauses vacuous)
    vacuous vote offset: -5  (1 positive-polarity, 6 negative-polarity vacuous)
  ============================================================
  
  misprediction log: appended 1843 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
  saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
  oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
    --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
    --preset p0 --engine byte-bag \
    --clauses 400 --vote-threshold 105 --states 85 \
    --specificity 4.2 --vocab-size 12000 --ngram-len 7 \
    --max-scan 4096 --epochs 12 --seed 248 --workers auto \
    --train-percent 80 \
    --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
    --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
      Finished `release` profile [optimized] target(s) in 0.01s
       Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 12000 --ngram-len 7 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
  loaded 49570 labeled documents
  resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 12000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }
  
  ============================================================
                 Classification Evaluation Report             
  ============================================================
    Run Preset:        p0           (Engine: byte-bag)
    Train/Test Split:  39656/9914 samples
  Training Time Duration (h:m:s): 00:33:48
  ------------------------------------------------------------
    Accuracy (@ V > 0): 83.70%
    Best-F1 Threshold:  V > -2
    Precision:          0.8233
    Recall:             0.8560
    F1-Score:           0.8394
  ------------------------------------------------------------
  Confusion Matrix (at optimal threshold):
                    Pred Neg (0)Pred Pos (1)
  Actual Neg (0)    4069        907         
  Actual Pos (1)    711         4227        
  ------------------------------------------------------------
  Clause Dynamics:
    fire-rate over 9914 test docs: never 0/400  always 4/400 (4 vacuous, 0 specialized)  p25 18.8%  median 21.4%  p75 24.3%
    includes/clause: min 0  p25 68  median 77  p75 88  max 132  (4 clauses vacuous)
    vacuous vote offset: -2  (1 positive-polarity, 3 negative-polarity vacuous)
  ============================================================
  
  misprediction log: appended 1616 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
  saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb

--vocab-size 10000
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10500 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.3 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.3 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.3, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:45:41
------------------------------------------------------------
  Accuracy (@ V > 0): 83.95%
  Best-F1 Threshold:  V > 1
  Precision:          0.8166
  Recall:             0.8882
  F1-Score:           0.8509
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3991        985         
Actual Pos (1)    552         4386        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.8%  median 21.1%  p75 23.6%
  includes/clause: min 1  p25 51  median 61  p75 69  max 98  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1591 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.3 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.3 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.3, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:45:29
------------------------------------------------------------
  Accuracy (@ V > 0): 85.01%
  Best-F1 Threshold:  V > -1
  Precision:          0.8582
  Recall:             0.8399
  F1-Score:           0.8489
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4237        692         
Actual Pos (1)    798         4187        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.4%  median 21.4%  p75 23.8%
  includes/clause: min 1  p25 53  median 60  p75 67  max 89  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1486 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.3 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.3 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.3, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:45:41
------------------------------------------------------------
  Accuracy (@ V > 0): 84.77%
  Best-F1 Threshold:  V > 0
  Precision:          0.8457
  Recall:             0.8508
  F1-Score:           0.8482
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4185        770         
Actual Pos (1)    740         4219        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.0%  median 20.0%  p75 22.0%
  includes/clause: min 1  p25 56  median 64  p75 73  max 106  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1510 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
...


oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10500 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 10500 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10500, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:43:17
------------------------------------------------------------
  Accuracy (@ V > 0): 84.32%
  Best-F1 Threshold:  V > 0
  Precision:          0.8094
  Recall:             0.8961
  F1-Score:           0.8506
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3934        1042        
Actual Pos (1)    513         4425        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.3%  median 21.7%  p75 23.9%
  includes/clause: min 1  p25 54  median 61  p75 71  max 108  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1555 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 


oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10500 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 105 --states 85 --specificity 4.2 --vocab-size 10500 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10500, n_clauses: 400, vote_threshold: 105, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:43:24
------------------------------------------------------------
  Accuracy (@ V > 0): 84.72%
  Best-F1 Threshold:  V > -2
  Precision:          0.8573
  Recall:             0.8349
  F1-Score:           0.8459
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4236        693         
Actual Pos (1)    823         4162        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.4%  median 21.6%  p75 23.8%
  includes/clause: min 1  p25 56  median 62  p75 69  max 96  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1515 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$

fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 110 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 110 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 110, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:44:59
------------------------------------------------------------
  Accuracy (@ V > 0): 84.72%
  Best-F1 Threshold:  V > 0
  Precision:          0.8354
  Recall:             0.8649
  F1-Score:           0.8499
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4110        845         
Actual Pos (1)    670         4289        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.0%  median 20.0%  p75 22.8%
  includes/clause: min 1  p25 56  median 64  p75 73  max 97  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1515 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 


$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 110 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 110 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 110, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:45:08
------------------------------------------------------------
  Accuracy (@ V > 0): 84.98%
  Best-F1 Threshold:  V > -1
  Precision:          0.8486
  Recall:             0.8497
  F1-Score:           0.8492
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4173        756         
Actual Pos (1)    749         4236        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 19.2%  median 21.7%  p75 24.1%
  includes/clause: min 2  p25 54  median 61  p75 67  max 97  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1489 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
saved model artifact to /home/oops/models/imdb_p0_c400_m10k.gmb
$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 400 --vote-threshold 110 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c400_m10k.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 400 --vote-threshold 110 --states 85 --specificity 4.2 --vocab-size 10000 --ngram-len 6 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c400_m10k.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c400_m10k.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 6, bag_vocab_size: 10000, n_clauses: 400, vote_threshold: 110, states_per_action: 85, specificity: 4.2, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:45:10
------------------------------------------------------------
  Accuracy (@ V > 0): 84.06%
  Best-F1 Threshold:  V > 0
  Precision:          0.8025
  Recall:             0.9020
  F1-Score:           0.8494
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3880        1096        
Actual Pos (1)    484         4454        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/400  always 0/400 (0 vacuous, 0 specialized)  p25 18.9%  median 21.7%  p75 24.5%
  includes/clause: min 1  p25 51  median 60  p75 69  max 110  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================


cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt


--states 85
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 87 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt


  --clauses 300
  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 294 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt

oops@fedora:~/code/granmo_model_nlp_classifier_rust/window_nlp_tests$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --text-col text \
  --label-col label \
  --jsonl \
  --model-type flat \
  --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json \
  --epochs 2 \
  --clauses 200 \
  --threshold 90 \
  --specificity 5 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.04s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --text-col text --label-col label --jsonl --model-type flat --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json --epochs 2 --clauses 200 --threshold 90 --specificity 5 --max-features 4000`
Loading training dataset from: /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl
  Total records loaded: 49570
Splitting dataset (49570 total rows) into 80% train / 20% test...
  Split: 39656 train rows, 9914 test rows
[1/3] Building vocabulary across 39656 documents...
  Active vocabulary features: 4000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (2 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 9914
  Training Time:     299.52s
  Accuracy:        74.00%
  Macro Precision: 0.7685
  Macro Recall:    0.7396
  Macro F1-Score:  0.7326
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               0             1             
0              2848          2097          
1              481           4488          
============================================================

Successfully saved trained model artifact to: /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json
oops@fedora:~/code/granmo_model_nlp_classifier_rust/window_nlp_tests$ cargo run --release --   --mode train   --train /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl   --text-col text   --label-col label   --jsonl   --model-type flat   --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json   --epochs 6   --clauses 200   --threshold 90   --specificity 5   --max-features 6000
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --text-col text --label-col label --jsonl --model-type flat --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json --epochs 6 --clauses 200 --threshold 90 --specificity 5 --max-features 6000`
Loading training dataset from: /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl
  Total records loaded: 49570
Splitting dataset (49570 total rows) into 80% train / 20% test...
  Split: 39656 train rows, 9914 test rows
[1/3] Building vocabulary across 39656 documents...
  Active vocabulary features: 6000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (6 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 9914
  Training Time:     2032.04s
  Accuracy:        69.75%
  Macro Precision: 0.7752
  Macro Recall:    0.6969
  Macro F1-Score:  0.6741
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               0             1             
0              2129          2816          
1              183           4786          
============================================================

Successfully saved trained model artifact to: /home/oops/models/Cyber_Bully_Data_binary_class-v4-model.json
oops@fedora:~/code/granmo_model_nlp_classifier_rust/window_nlp_tests$ 


oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 294 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \ 
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 294 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 294, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:25
------------------------------------------------------------
  Accuracy (@ V > 0): 84.56%
  Best-F1 Threshold:  V > 0
  Precision:          0.8357
  Recall:             0.8605
  F1-Score:           0.8479
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4116        839         
Actual Pos (1)    692         4267        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/294  always 0/294 (0 vacuous, 0 specialized)  p25 21.5%  median 23.6%  p75 26.2%
  includes/clause: min 1  p25 33  median 40  p75 46  max 63  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1531 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 294 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \ 
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 294 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 294, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:25
------------------------------------------------------------
  Accuracy (@ V > 0): 84.56%
  Best-F1 Threshold:  V > 0
  Precision:          0.8357
  Recall:             0.8605
  F1-Score:           0.8479
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4116        839         
Actual Pos (1)    692         4267        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/294  always 0/294 (0 vacuous, 0 specialized)  p25 21.5%  median 23.6%  p75 26.2%
  includes/clause: min 1  p25 33  median 40  p75 46  max 63  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1531 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 

oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$   --clauses 300
  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 294 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
bash: --clauses: command not found...
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 294 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 294, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:20
------------------------------------------------------------
  Accuracy (@ V > 0): 84.44%
  Best-F1 Threshold:  V > 1
  Precision:          0.8410
  Recall:             0.8650
  F1-Score:           0.8528
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4114        815         
Actual Pos (1)    673         4312        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/294  always 2/294 (2 vacuous, 0 specialized)  p25 23.1%  median 25.2%  p75 27.8%
  includes/clause: min 0  p25 32  median 37  p75 42  max 62  (2 clauses vacuous)
  vacuous vote offset: +2  (2 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1543 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 


oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$   cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 294 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 294 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 294, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:14
------------------------------------------------------------
  Accuracy (@ V > 0): 83.92%
  Best-F1 Threshold:  V > 1
  Precision:          0.8175
  Recall:             0.8870
  F1-Score:           0.8508
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3998        978         
Actual Pos (1)    558         4380        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/294  always 0/294 (0 vacuous, 0 specialized)  p25 21.8%  median 24.6%  p75 28.0%
  includes/clause: min 1  p25 30  median 38  p75 45  max 63  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1594 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 

oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 87 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 87 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 87, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:20:41
------------------------------------------------------------
  Accuracy (@ V > 0): 84.83%
  Best-F1 Threshold:  V > 0
  Precision:          0.8391
  Recall:             0.8621
  F1-Score:           0.8504
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4135        820         
Actual Pos (1)    684         4275        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 0/300 (0 vacuous, 0 specialized)  p25 21.3%  median 23.5%  p75 25.9%
  includes/clause: min 1  p25 33  median 40  p75 47  max 64  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1504 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 

ps@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 87 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 87 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 87, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:11
------------------------------------------------------------
  Accuracy (@ V > 0): 84.93%
  Best-F1 Threshold:  V > 0
  Precision:          0.8427
  Recall:             0.8610
  F1-Score:           0.8518
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4128        801         
Actual Pos (1)    693         4292        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 1/300 (1 vacuous, 0 specialized)  p25 22.9%  median 25.5%  p75 27.8%
  includes/clause: min 0  p25 31  median 37  p75 41  max 54  (1 clauses vacuous)
  vacuous vote offset: +1  (1 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1494 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ 

dora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 87 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 248 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 300 --vote-threshold 75 --states 87 --specificity 3.7 --vocab-size 8000 --ngram-len 5 --max-scan 4096 --epochs 12 --seed 248 --workers auto --train-percent 80 --model-out /home/oops/models/imdb_p0_c300_e12.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 8000, n_clauses: 300, vote_threshold: 75, states_per_action: 87, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 12, seed: 248, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
Training Time Duration (h:m:s): 00:21:20
------------------------------------------------------------
  Accuracy (@ V > 0): 83.75%
  Best-F1 Threshold:  V > 2
  Precision:          0.8287
  Recall:             0.8702
  F1-Score:           0.8490
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    4088        888         
Actual Pos (1)    641         4297        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/300  always 1/300 (1 vacuous, 0 specialized)  p25 22.1%  median 24.9%  p75 28.0%
  includes/clause: min 0  p25 31  median 37  p75 44  max 73  (1 clauses vacuous)
  vacuous vote offset: +1  (1 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1611 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt
saved model artifact to /home/oops/models/imdb_p0_c300_e12.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/ensemble_granmo$


 --clauses 300
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 290 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt

--states 85
  cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 84 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/imdb_p0_c300_e12.txt



e.g.
(C=400, T=100, M=10k, s=4.2)

--clauses 400) — "C" Clause Bank Capacity
--vote-threshold 100) — "T" The Margin / Confidence Ceiling
--vocab-size 10000) — "M" Feature Space / Vocabulary Size
--specificity 4.2) — "s" The Pattern Strictness Knob

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \
  --max-scan 4096 --epochs 12 --seed 128 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdb_p0_c300_e12.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/models/imdb_p0_ngrm5_c300_T75_M8k_s3_7.gmb
