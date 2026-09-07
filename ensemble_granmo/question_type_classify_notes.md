for deterministic-synthetic dataset generation 

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

$ cargo run --release -- --mode train   --data /home/oops/code/deterministic_nlp_dataset_generators_py/question_type_dataset_output/balanced_unified_question_type_dataset_v2.jsonl  --preset p0 --engine byte-bag   --clauses 110 --vote-threshold 71 --states 85  --specificity 3.7 --vocab-size 2800 --ngram-len 7   --max-scan 4096 --epochs 3 --seed 128 --workers auto   --train-percent 80  --model-out /home/oops/models/unified_question_type_dataset_v2.gmb  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/ensemble_granmo/logs/question_type_v2.txt
