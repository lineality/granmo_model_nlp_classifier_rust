
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



cargo run --release -- --mode train \
  --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/unified_question_type_dataset.jsonl \
  --label-key intent_classification_label \
  --text-key query_text \
  --preset p0 --engine byte-bag \
  --clauses 100 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 2000 --ngram-len 4 \
  --max-scan 4096 --epochs 3 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/unified_question_type_datasetv1.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt


cargo run --release -- --mode train \
  --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/unified_question_type_dataset.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 100 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 2000 --ngram-len 4 \
  --max-scan 4096 --epochs 3 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/unified_question_type_datasetv1.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt


  cargo run --release -- --mode train \
  --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/unified_question_type_dataset.jsonl \
  --preset p0 --engine byte-bag \
  --clauses 100 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 4000 --ngram-len 5 \
  --max-scan 4096 --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/unified_question_type_datasetv1.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt


  $ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset.jsonl   --preset p0 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 2000 --ngram-len 4   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset.jsonl   --preset p0 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 2000 --ngram-len 4   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset.jsonl --preset p0 --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 2000 --ngram-len 4 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 8040 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 2000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  6432/1608 samples
Training Time Duration (h:m:s): 00:00:03
------------------------------------------------------------
  Accuracy (@ V > 0): 99.38%
  Best-F1 Threshold:  V > 0
  Precision:          1.0000
  Recall:             0.9876
  F1-Score:           0.9938
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    801         0           
Actual Pos (1)    10          797         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 1608 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 12.8%  median 13.9%  p75 15.0%
  includes/clause: min 83  p25 261  median 322  p75 404  max 541  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 10 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl   --preset p5 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 2000 --ngram-len 4   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl --preset p5 --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 2000 --ngram-len 4 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 8040 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 143 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 2000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p5           (Engine: byte-bag)
  Train/Test Split:  6432/1608 samples
Training Time Duration (h:m:s): 00:00:03
------------------------------------------------------------
  Accuracy (@ V > 0): 99.56%
  Best-F1 Threshold:  V > 0
  Precision:          1.0000
  Recall:             0.9913
  F1-Score:           0.9956
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    801         0           
Actual Pos (1)    7           800         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 1608 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 13.0%  median 14.0%  p75 15.2%
  includes/clause: min 67  p25 333  median 384  p75 436  max 665  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 7 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...


cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl   --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 2000 --ngram-len 8   --max-scan 4096 --epochs 4 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt


...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl   --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 2000 --ngram-len 8   --max-scan 4096 --epochs 4 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl --preset raw --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 2000 --ngram-len 8 --max-scan 4096 --epochs 4 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 8040 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 8, bag_vocab_size: 2000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 4, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  6432/1608 samples
Training Time Duration (h:m:s): 00:00:05
------------------------------------------------------------
  Accuracy (@ V > 0): 99.94%
  Best-F1 Threshold:  V > 0
  Precision:          1.0000
  Recall:             0.9988
  F1-Score:           0.9994
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    801         0           
Actual Pos (1)    1           806         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 1608 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 13.8%  median 14.8%  p75 15.5%
  includes/clause: min 126  p25 331  median 396  p75 445  max 545  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...

cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl   --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 2000 --ngram-len 9   --max-scan 4096 --epochs 5 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt


cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl   --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 8   --max-scan 4096 --epochs 4 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt



....

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl   --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 8   --max-scan 4096 --epochs 4 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v1.jsonl --preset raw --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 3000 --ngram-len 8 --max-scan 4096 --epochs 4 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 8040 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 8, bag_vocab_size: 3000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 4, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  6432/1608 samples
Training Time Duration (h:m:s): 00:00:05
------------------------------------------------------------
  Accuracy (@ V > 0): 99.94%
  Best-F1 Threshold:  V > 1
  Precision:          1.0000
  Recall:             1.0000
  F1-Score:           1.0000
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    801         0           
Actual Pos (1)    0           807         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 1608 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 12.7%  median 13.9%  p75 15.0%
  includes/clause: min 267  p25 475  median 539  p75 619  max 870  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb


....

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 8   --max-scan 4096 --epochs 4 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset raw --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 3000 --ngram-len 8 --max-scan 4096 --epochs 4 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 8, bag_vocab_size: 3000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 4, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:02:23
------------------------------------------------------------
  Accuracy (@ V > 0): 98.70%
  Best-F1 Threshold:  V > 0
  Precision:          0.9997
  Recall:             0.9741
  F1-Score:           0.9867
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20081       5           
Actual Pos (1)    514         19323       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 15.0%  median 15.8%  p75 16.5%
  includes/clause: min 410  p25 532  median 679  p75 1037  max 1276  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 519 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb


...
$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset raw --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset raw --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 3000 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 3000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:48
------------------------------------------------------------
  Accuracy (@ V > 0): 99.47%
  Best-F1 Threshold:  V > 0
  Precision:          0.9997
  Recall:             0.9896
  F1-Score:           0.9946
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20081       5           
Actual Pos (1)    207         19630       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 14.9%  median 15.6%  p75 16.3%
  includes/clause: min 450  p25 622  median 719  p75 797  max 1064  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 212 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p5 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p5 --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 3000 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 143 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 3000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p5           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:49
------------------------------------------------------------
  Accuracy (@ V > 0): 97.77%
  Best-F1 Threshold:  V > -1
  Precision:          0.9647
  Recall:             0.9956
  F1-Score:           0.9799
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    19363       723         
Actual Pos (1)    88          19749       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 15.0%  median 15.9%  p75 16.7%
  includes/clause: min 446  p25 594  median 746  p75 1000  max 1256  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 890 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 7   --max-scan 4096 --epochs 2 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 3000 --ngram-len 7 --max-scan 4096 --epochs 2 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 3000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 2, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:06
------------------------------------------------------------
  Accuracy (@ V > 0): 99.45%
  Best-F1 Threshold:  V > 0
  Precision:          0.9993
  Recall:             0.9897
  F1-Score:           0.9945
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20073       13          
Actual Pos (1)    205         19632       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 14.7%  median 15.4%  p75 15.9%
  includes/clause: min 464  p25 627  median 704  p75 804  max 1016  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 218 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 3000 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 3000, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:52
------------------------------------------------------------
  Accuracy (@ V > 0): 99.50%
  Best-F1 Threshold:  V > 0
  Precision:          0.9996
  Recall:             0.9903
  F1-Score:           0.9950
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20079       7           
Actual Pos (1)    192         19645       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 14.8%  median 15.5%  p75 16.3%
  includes/clause: min 461  p25 635  median 729  p75 817  max 992  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 199 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...

cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85   --specificity 3.7 --vocab-size 3000 --ngram-len 7   --max-scan 4096 --epochs 2 --seed 128 --workers auto   --train-percent 80   --model-out /home/oops/models/unified_question_type_datasetv1.gmb   --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/trash.txt

...
$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 100 --vote-threshold 75 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 100 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 100, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:45
------------------------------------------------------------
  Accuracy (@ V > 0): 99.71%
  Best-F1 Threshold:  V > 0
  Precision:          0.9996
  Recall:             0.9946
  F1-Score:           0.9971
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20078       8           
Actual Pos (1)    107         19730       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/100  always 0/100 (0 vacuous, 0 specialized)  p25 14.7%  median 15.4%  p75 16.3%
  includes/clause: min 406  p25 589  median 677  p75 832  max 1036  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 115 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 75 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 110 --vote-threshold 75 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 110, vote_threshold: 75, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:55
------------------------------------------------------------
  Accuracy (@ V > 0): 99.80%
  Best-F1 Threshold:  V > 0
  Precision:          0.9996
  Recall:             0.9963
  F1-Score:           0.9980
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20079       7           
Actual Pos (1)    74          19763       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/110  always 0/110 (0 vacuous, 0 specialized)  p25 15.0%  median 15.6%  p75 16.1%
  includes/clause: min 430  p25 595  median 696  p75 812  max 1012  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 81 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb


...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 73 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 110 --vote-threshold 73 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 110, vote_threshold: 73, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:01:56
------------------------------------------------------------
  Accuracy (@ V > 0): 99.84%
  Best-F1 Threshold:  V > 0
  Precision:          0.9996
  Recall:             0.9972
  F1-Score:           0.9984
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20079       7           
Actual Pos (1)    56          19781       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/110  always 0/110 (0 vacuous, 0 specialized)  p25 14.9%  median 15.6%  p75 16.4%
  includes/clause: min 417  p25 601  median 695  p75 825  max 1032  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 63 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb

...
$ python /home/oops/code/deterministic_nlp_dataset_generators_py/time_event_binary_classifier_gen_v5.py --mode unified --balanced --standard-format
[PIPELINE STATUS] Generating Class A records (KNOWN_EVENT_UNKNOWN_TIME) with prefix modifiers...
[PIPELINE STATUS] Generated 132392 records for Class A.
[PIPELINE STATUS] Generating Class B records (KNOWN_TIME_UNKNOWN_EVENT) with systematic numbers...
[PIPELINE STATUS] Generated 99807 records for Class B.
[PIPELINE STATUS] Balancing classes to identical record cardinality...
[PIPELINE STATUS] Balanced dataset total records: 199614
[PIPELINE STATUS] Projecting records to standard format schema ('text', 'label')...
[SUCCESS] Wrote Unified Set: question_type_dataset_output/unified_question_type_dataset.jsonl
          Total: 199614 | Class A: 99807 | Class B: 99807
[PIPELINE STATUS] K-Fold CV Folds Assigned: 0 through 4

...
$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 73 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 256 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 110 --vote-threshold 73 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 256 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 110, vote_threshold: 73, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 256, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:02:28
------------------------------------------------------------
  Accuracy (@ V > 0): 99.80%
  Best-F1 Threshold:  V > 0
  Precision:          1.0000
  Recall:             0.9959
  F1-Score:           0.9980
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    19941       0           
Actual Pos (1)    81          19901       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/110  always 0/110 (0 vacuous, 0 specialized)  p25 14.6%  median 15.4%  p75 16.2%
  includes/clause: min 418  p25 619  median 692  p75 830  max 1017  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 81 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb
...
$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 71 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 256 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 110 --vote-threshold 71 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 256 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 110, vote_threshold: 71, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 256, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:03:52
------------------------------------------------------------
  Accuracy (@ V > 0): 99.82%
  Best-F1 Threshold:  V > 0
  Precision:          0.9998
  Recall:             0.9965
  F1-Score:           0.9982
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    19937       4           
Actual Pos (1)    69          19913       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/110  always 0/110 (0 vacuous, 0 specialized)  p25 14.6%  median 15.4%  p75 16.3%
  includes/clause: min 390  p25 636  median 713  p75 835  max 1197  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 73 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb
...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 71 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 42 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 110 --vote-threshold 71 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 110, vote_threshold: 71, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:04:05
------------------------------------------------------------
  Accuracy (@ V > 0): 99.86%
  Best-F1 Threshold:  V > 0
  Precision:          0.9999
  Recall:             0.9972
  F1-Score:           0.9986
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    19955       2           
Actual Pos (1)    55          19911       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/110  always 0/110 (0 vacuous, 0 specialized)  p25 15.1%  median 15.9%  p75 16.7%
  includes/clause: min 398  p25 575  median 691  p75 824  max 1031  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 57 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb
...

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 71 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/ensemble_granmo --mode train --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl --preset p0 --engine byte-bag --clauses 110 --vote-threshold 71 --states 85 --specificity 3.7 --vocab-size 2800 --ngram-len 7 --max-scan 4096 --epochs 3 --seed 128 --workers auto --train-percent 80 --model-out /home/oops/models/unified_question_type_datasetv1.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt`
loaded 199614 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 7, bag_vocab_size: 2800, n_clauses: 110, vote_threshold: 71, states_per_action: 85, specificity: 3.7, max_scan_bytes: 4096, guarded_include: false, fire_guard_streak_limit: 0, epochs: 3, seed: 128, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  159691/39923 samples
Training Time Duration (h:m:s): 00:02:27
------------------------------------------------------------
  Accuracy (@ V > 0): 99.89%
  Best-F1 Threshold:  V > 0
  Precision:          0.9997
  Recall:             0.9981
  F1-Score:           0.9989
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    20080       6           
Actual Pos (1)    37          19800       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 39923 test docs: never 0/110  always 0/110 (0 vacuous, 0 specialized)  p25 14.8%  median 15.6%  p75 16.5%
  includes/clause: min 432  p25 569  median 661  p75 829  max 1059  (0 clauses vacuous)
  vacuous vote offset: +0  (0 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 43 records to /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt
saved model artifact to /home/oops/models/unified_question_type_datasetv1.gmb
...

cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 71 --states 86  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_datasetv1.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/trash.txt



```bash
cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 71 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_dataset_v2.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/question_type_v2.txt
```
