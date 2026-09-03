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
