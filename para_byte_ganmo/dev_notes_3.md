# Comparing Multiple Datasets...

Exmple of quick first check:
```bash
# Quick sanity check on a new dataset
cargo run --release -- --mode train \
  --data /path/to/new_dataset.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 50 --states 50 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/_.gmb \
  --log-out /home/oops/code/_.txt \
  ```

/////////////////////////////////

$ cargo run --release -- --mode train \
  --data  /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl \ 
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb \ 
  --log-out /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl ' '`
bash: --preset: command not found...
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data  /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb \ 
  --log-out /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb --log-out /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt`
loaded 27969 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  22375/5594 samples
  Training Time:     43.89s
------------------------------------------------------------
  Accuracy (@ V > 0): 49.39%
  Best-F1 Threshold:  V > 19
  Precision:          0.6334
  Recall:             0.8585
  F1-Score:           0.7289
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1458        1373        
Actual Pos (1)    391         2372        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 5594 test docs: never 55/100  always 21/100 (21 vacuous, 0 specialized)  p25 0.0%  median 0.0%  p75 26.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1280  (21 clauses vacuous)
  vacuous vote offset: +17  (19 positive-polarity, 2 negative-polarity vacuous)
============================================================

misprediction log: appended 2831 records to /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt
saved model artifact to /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train   --data  /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl   --preset p0   --clauses 100 --vote-threshold 60 --states 200   --specificity 3.0 --vocab-size 1000 --ngram-len 5   --epochs 10 --seed 42 --workers auto   --train-percent 80   --model-out /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb   --log-out /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 200 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 10 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb --log-out /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt`
loaded 27969 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 200, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 10, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  22375/5594 samples
  Training Time:     77.97s
------------------------------------------------------------
  Accuracy (@ V > 0): 50.20%
  Best-F1 Threshold:  V > 3
  Precision:          0.6685
  Recall:             0.8129
  F1-Score:           0.7336
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1717        1114        
Actual Pos (1)    517         2246        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 5594 test docs: never 58/100  always 24/100 (24 vacuous, 0 specialized)  p25 0.0%  median 0.0%  p75 26.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1280  (24 clauses vacuous)
  vacuous vote offset: +2  (13 positive-polarity, 11 negative-polarity vacuous)
============================================================

misprediction log: appended 2786 records to /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt
saved model artifact to /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb


///////////////////////////


(previous standard)
```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/TESTED/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json-cyberbullying-detect_v1.gmb \
  --log-out /home/oops/code/LOG_json-cyberbullying-detect_v1.txt
```

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/TESTED/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json-cyberbullying-detect_v1.gmb \
  --log-out /home/oops/code/LOG_json-cyberbullying-detect_v1.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/TESTED/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json-cyberbullying-detect_v1.gmb --log-out /home/oops/code/LOG_json-cyberbullying-detect_v1.txt`
loaded 99736 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     188.26s
------------------------------------------------------------
  Accuracy (@ V > 0): 58.20%
  Best-F1 Threshold:  V > -11
  Precision:          0.7095
  Recall:             0.7681
  F1-Score:           0.7377
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    6874        3128        
Actual Pos (1)    2306        7640        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 8/100  always 15/100 (15 vacuous, 0 specialized)  p25 12.5%  median 20.1%  p75 26.0%
  includes/clause: min 0  p25 1  median 1  p75 4  max 1247  (15 clauses vacuous)
  vacuous vote offset: -13  (1 positive-polarity, 14 negative-polarity vacuous)
============================================================

misprediction log: appended 8339 records to /home/oops/code/LOG_json-cyberbullying-detect_v1.txt
saved model artifact to /home/oops/models/model_json-cyberbullying-detect_v1.gmb

  


  ```bash
# Quick sanity check on a new dataset
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/mmumairkhattak-toxic-comments-dataset-2026-cm-and-nlp-archive/2026_toxic_comments_dataset_binaryclass_dedupe_v1.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 50 --states 50 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/mmumairkhattak-toxic-comments-dataset-2026v1.json \
  --log-out /home/oops/code/LOG_mmumairkhattak-toxic-comments-dataset-2026v1.txt
  ```

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/mmumairkhattak-toxic-comments-dataset-2026-cm-and-nlp-archive/2026_toxic_comments_dataset_binaryclass_dedupe_v1.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 50 --states 50 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/mmumairkhattak-toxic-comments-dataset-2026v1.json \
  --log-out /home/oops/code/LOG_mmumairkhattak-toxic-comments-dataset-2026v1.txt 
   Compiling para_byte_ganmo v0.1.0 (/home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo)
    Finished `release` profile [optimized] target(s) in 5.19s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/mmumairkhattak-toxic-comments-dataset-2026-cm-and-nlp-archive/2026_toxic_comments_dataset_binaryclass_dedupe_v1.jsonl --preset p0 --clauses 100 --vote-threshold 50 --states 50 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 2 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/mmumairkhattak-toxic-comments-dataset-2026v1.json --log-out /home/oops/code/LOG_mmumairkhattak-toxic-comments-dataset-2026v1.txt`
loaded 30000 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 50, states_per_action: 50, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 2, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  24000/6000 samples
  Training Time:     14.48s
------------------------------------------------------------
  Accuracy (@ V > 0): 100.00%
  Best-F1 Threshold:  V > -15
  Precision:          1.0000
  Recall:             1.0000
  F1-Score:           1.0000
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3703        0           
Actual Pos (1)    0           2297        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 6000 test docs: never 15/100  always 8/100 (7 vacuous, 1 specialized)  p25 15.2%  median 30.4%  p75 30.9%
  includes/clause: min 0  p25 1  median 1270  p75 1274  max 1280  (7 clauses vacuous)
  vacuous vote offset: +7  (7 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: no mispredictions this run (nothing appended)
saved model artifact to /home/oops/models/mmumairkhattak-toxic-comments-dataset-2026v1.json






```bash
# Quick sanity check on a new dataset
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary-subhajeetdas-hate-comment/binary-subhajeetdas-hate-comment_dedupe_v1.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 50 --states 50 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 2 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb \
  --log-out /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt
  ```
  
  $ cargo run --release -- --mode train \
    --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary-subhajeetdas-hate-comment/binary-subhajeetdas-hate-comment_dedupe_v1.jsonl \ 
    --preset p0 \
    --clauses 100 --vote-threshold 50 --states 50 \
    --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
    --epochs 2 --seed 42 --workers auto \
    --train-percent 80 \
    --model-out /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb \
    --log-out /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt
      Finished `release` profile [optimized] target(s) in 0.03s
       Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary-subhajeetdas-hate-comment/binary-subhajeetdas-hate-comment_dedupe_v1.jsonl --preset p0 --clauses 100 --vote-threshold 50 --states 50 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 2 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb --log-out /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt`
  loaded 40605 labeled documents
  resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 50, states_per_action: 50, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 2, seed: 42, worker_count: 16 }
  
  ============================================================
                 Classification Evaluation Report             
  ============================================================
    Run Preset:        p0           (Engine: byte-conv)
    Train/Test Split:  32484/8121 samples
    Training Time:     20.61s
  ------------------------------------------------------------
    Accuracy (@ V > 0): 46.99%
    Best-F1 Threshold:  V > -30
    Precision:          0.5301
    Recall:             1.0000
    F1-Score:           0.6929
  ------------------------------------------------------------
  Confusion Matrix (at optimal threshold):
                    Pred Neg (0)Pred Pos (1)
  Actual Neg (0)    0           3816        
  Actual Pos (1)    0           4305        
  ------------------------------------------------------------
  Clause Dynamics:
    fire-rate over 8121 test docs: never 51/100  always 35/100 (35 vacuous, 0 specialized)  p25 0.0%  median 0.0%  p75 100.0%
    includes/clause: min 0  p25 0  median 1  p75 1  max 3  (35 clauses vacuous)
    vacuous vote offset: -27  (4 positive-polarity, 31 negative-polarity vacuous)
  ============================================================



cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary-subhajeetdas-hate-comment/binary-subhajeetdas-hate-comment_dedupe_v1.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb \
  --log-out /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt


$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary-subhajeetdas-hate-comment/binary-subhajeetdas-hate-comment_dedupe_v1.jsonl \ 
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb \
  --log-out /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary-subhajeetdas-hate-comment/binary-subhajeetdas-hate-comment_dedupe_v1.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb --log-out /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt`
loaded 40605 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  32484/8121 samples
  Training Time:     63.11s
------------------------------------------------------------
  Accuracy (@ V > 0): 53.01%
  Best-F1 Threshold:  V > 4
  Precision:          0.5302
  Recall:             1.0000
  F1-Score:           0.6930
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    2           3814        
Actual Pos (1)    0           4305        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 8121 test docs: never 57/100  always 24/100 (24 vacuous, 0 specialized)  p25 0.0%  median 0.0%  p75 27.8%
  includes/clause: min 0  p25 1  median 1  p75 1  max 72  (24 clauses vacuous)
  vacuous vote offset: +6  (15 positive-polarity, 9 negative-polarity vacuous)
============================================================

misprediction log: appended 3816 records to /home/oops/code/LOG_binary-subhajeetdas-hate-comment_dedupe_v1.txt
saved model artifact to /home/oops/models/model_json_binary-subhajeetdas-hate-comment_dedupe_v1.gmb

/////////////

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english_v3_dedupe.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_toxic_comments_english_v3.gmb \
  --log-out /home/oops/code/LOG_toxic_comments_english_v3.txt

$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english_v3_dedupe.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_toxic_comments_english_v3.gmb \
  --log-out /home/oops/code/LOG_toxic_comments_english_v3.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english_v3_dedupe.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_toxic_comments_english_v3.gmb --log-out /home/oops/code/LOG_toxic_comments_english_v3.txt`
loaded 10000 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  8000/2000 samples
  Training Time:     15.00s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.95%
  Best-F1 Threshold:  V > -3
  Precision:          1.0000
  Recall:             1.0000
  F1-Score:           1.0000
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1039        0           
Actual Pos (1)    0           961         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2000 test docs: never 2/100  always 2/100 (1 vacuous, 1 specialized)  p25 16.0%  median 17.6%  p75 19.9%
  includes/clause: min 0  p25 2  median 836  p75 1259  max 1271  (1 clauses vacuous)
  vacuous vote offset: -1  (0 positive-polarity, 1 negative-polarity vacuous)
============================================================

misprediction log: appended 41 records to /home/oops/code/LOG_toxic_comments_english_v3.txt
saved model artifact to /home/oops/models/model_json_toxic_comments_english_v3.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english_v3_dedupe.jsonl \
  --preset p0 \
  --clauses 80 --vote-threshold 40 --states 80 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_toxic_comments_english_v4sm.gmb \
  --log-out /home/oops/code/LOG_toxic_comments_english_v4sm.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english_v3_dedupe.jsonl --preset p0 --clauses 80 --vote-threshold 40 --states 80 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 4 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_toxic_comments_english_v4sm.gmb --log-out /home/oops/code/LOG_toxic_comments_english_v4sm.txt`
loaded 10000 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 80, vote_threshold: 40, states_per_action: 80, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 4, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  8000/2000 samples
  Training Time:     10.45s
------------------------------------------------------------
  Accuracy (@ V > 0): 90.10%
  Best-F1 Threshold:  V > 6
  Precision:          1.0000
  Recall:             1.0000
  F1-Score:           1.0000
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1039        0           
Actual Pos (1)    0           961         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2000 test docs: never 7/80  always 8/80 (6 vacuous, 2 specialized)  p25 14.1%  median 18.0%  p75 20.9%
  includes/clause: min 0  p25 1  median 5  p75 1112  max 1266  (6 clauses vacuous)
  vacuous vote offset: +6  (6 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 198 records to /home/oops/code/LOG_toxic_comments_english_v4sm.txt
saved model artifact to /home/oops/models/model_json_toxic_comments_english_v4sm.gmb

```


```bash
cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english_v3_dedupe.jsonl \
  --preset p0 \
  --clauses 80 --vote-threshold 40 --states 80 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_toxic_comments_english_v4sm.gmb \
  --log-out /home/oops/code/LOG_toxic_comments_english_v4sm.txt
```

////////////////

cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/binary_class_sets/binary_or_multiclass-cyberbullying-classification/andrewmvd-cyberbullying-classification_tweets_dedupe_v2.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_andrewmvd-cyberbullying_v2.gmb \
  --log-out /home/oops/code/LOG_andrewmvd-cyberbullying_v2.txt

//////////////



cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p1  --engine byte-bag \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt

  
//////////////


cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p1  --engine byte-bag \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt



//////////////
