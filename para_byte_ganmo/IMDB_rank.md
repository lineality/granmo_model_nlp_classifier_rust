
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




...


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
