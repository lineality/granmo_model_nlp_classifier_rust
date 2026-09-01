
misprediction log: appended 1640 records to /home/oops/code/LOG_andrewmvd-cyberbullying_v2.txt
saved model artifact to /home/oops/models/model_json_andrewmvd-cyberbullying_v2.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt`
loaded 13619 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  10895/2724 samples
  Training Time:     27.40s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.23%
  Best-F1 Threshold:  V > -14
  Precision:          0.4507
  Recall:             0.9631
  F1-Score:           0.6141
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    194         1366        
Actual Pos (1)    43          1121        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 40/100  always 24/100 (24 vacuous, 0 specialized)  p25 0.0%  median 0.2%  p75 33.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1280  (24 clauses vacuous)
  vacuous vote offset: -2  (11 positive-polarity, 13 negative-polarity vacuous)
============================================================

misprediction log: appended 1165 records to /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train   --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl   --preset p0   --clauses 200 --vote-threshold 60 --states 200   --specificity 3.0 --vocab-size 2000 --ngram-len 5   --epochs 9 --seed 42 --workers auto   --train-percent 80   --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb   --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p0 --clauses 200 --vote-threshold 60 --states 200 --specificity 3.0 --vocab-size 2000 --ngram-len 5 --epochs 9 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt`
loaded 13619 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 2000, n_clauses: 200, vote_threshold: 60, states_per_action: 200, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 9, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  10895/2724 samples
  Training Time:     38.25s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.27%
  Best-F1 Threshold:  V > -21
  Precision:          0.4543
  Recall:             0.9656
  F1-Score:           0.6179
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    210         1350        
Actual Pos (1)    40          1124        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 87/200  always 55/200 (55 vacuous, 0 specialized)  p25 0.0%  median 0.1%  p75 100.0%
  includes/clause: min 0  p25 0  median 1  p75 1  max 1110  (55 clauses vacuous)
  vacuous vote offset: -9  (23 positive-polarity, 32 negative-polarity vacuous)
============================================================

misprediction log: appended 1164 records to /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p3  --engine byte-bag \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p3 --engine byte-bag --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt`
loaded 13619 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 95 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p3           (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     14.59s
------------------------------------------------------------
  Accuracy (@ V > 0): 62.30%
  Best-F1 Threshold:  V > -18
  Precision:          0.4571
  Recall:             0.9012
  F1-Score:           0.6065
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    314         1246        
Actual Pos (1)    115         1049        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 0/100  always 14/100 (14 vacuous, 0 specialized)  p25 8.1%  median 31.6%  p75 36.0%
  includes/clause: min 0  p25 1  median 1  p75 48  max 60  (14 clauses vacuous)
  vacuous vote offset: +14  (14 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1027 records to /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p1  --engine byte-bag \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p1 --engine byte-bag --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt`
loaded 13619 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 7 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     11.67s
------------------------------------------------------------
  Accuracy (@ V > 0): 66.15%
  Best-F1 Threshold:  V > -24
  Precision:          0.4548
  Recall:             0.9124
  F1-Score:           0.6070
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    287         1273        
Actual Pos (1)    102         1062        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 0/100  always 8/100 (8 vacuous, 0 specialized)  p25 8.1%  median 30.4%  p75 34.9%
  includes/clause: min 0  p25 1  median 2  p75 46  max 58  (8 clauses vacuous)
  vacuous vote offset: +8  (8 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 922 records to /home/oops/code/LOG_sayankr007-cyber_offensivev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_offensivev1.gmb



oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p0 \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
    Finished `release` profile [optimized] target(s) in 0.12s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt`
loaded 14294 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     28.28s
------------------------------------------------------------
  Accuracy (@ V > 0): 45.02%
  Best-F1 Threshold:  V > 9
  Precision:          0.5522
  Recall:             0.8022
  F1-Score:           0.6541
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    748         832         
Actual Pos (1)    253         1026        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 40/100  always 22/100 (22 vacuous, 0 specialized)  p25 0.0%  median 0.1%  p75 26.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1274  (22 clauses vacuous)
  vacuous vote offset: +12  (17 positive-polarity, 5 negative-polarity vacuous)
============================================================

misprediction log: appended 1572 records to /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train   --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl   --preset p0   --clauses 100 --vote-threshold 60 --states 120   --specificity 3.0 --vocab-size 1000 --ngram-len 5   --epochs 9 --seed 42 --workers auto   --train-percent 80   --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb   --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p0 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 9 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt`
loaded 14294 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 9, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     33.16s
------------------------------------------------------------
  Accuracy (@ V > 0): 55.26%
  Best-F1 Threshold:  V > -20
  Precision:          0.5319
  Recall:             0.8616
  F1-Score:           0.6577
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         970         
Actual Pos (1)    177         1102        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 37/100  always 28/100 (28 vacuous, 0 specialized)  p25 0.0%  median 0.7%  p75 100.0%
  includes/clause: min 0  p25 0  median 1  p75 1  max 1172  (28 clauses vacuous)
  vacuous vote offset: -16  (6 positive-polarity, 22 negative-polarity vacuous)
============================================================

misprediction log: appended 1279 records to /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train   --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl   --preset p1   --clauses 100 --vote-threshold 60 --states 120   --specificity 3.0 --vocab-size 1000 --ngram-len 5   --epochs 9 --seed 42 --workers auto   --train-percent 80   --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb   --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p1 --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 9 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt`
loaded 14294 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 7 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 9, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     34.48s
------------------------------------------------------------
  Accuracy (@ V > 0): 55.26%
  Best-F1 Threshold:  V > -20
  Precision:          0.5319
  Recall:             0.8616
  F1-Score:           0.6577
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         970         
Actual Pos (1)    177         1102        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 37/100  always 28/100 (28 vacuous, 0 specialized)  p25 0.0%  median 0.7%  p75 100.0%
  includes/clause: min 0  p25 0  median 1  p75 1  max 1172  (28 clauses vacuous)
  vacuous vote offset: -16  (6 positive-polarity, 22 negative-polarity vacuous)
============================================================

misprediction log: appended 1279 records to /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p3  --engine byte-bag \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p3 --engine byte-bag --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt`
loaded 14294 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 95 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p3           (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     15.29s
------------------------------------------------------------
  Accuracy (@ V > 0): 75.27%
  Best-F1 Threshold:  V > 1
  Precision:          0.7346
  Recall:             0.7271
  F1-Score:           0.7308
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1244        336         
Actual Pos (1)    349         930         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/100  always 9/100 (9 vacuous, 0 specialized)  p25 15.1%  median 27.2%  p75 30.2%
  includes/clause: min 0  p25 1  median 6  p75 59  max 72  (9 clauses vacuous)
  vacuous vote offset: +9  (9 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 707 records to /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release -- --mode train \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --preset p1  --engine byte-bag \
  --clauses 100 --vote-threshold 60 --states 120 \
  --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 6 --seed 42 --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb \
  --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --preset p1 --engine byte-bag --clauses 100 --vote-threshold 60 --states 120 --specificity 3.0 --vocab-size 1000 --ngram-len 5 --epochs 6 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb --log-out /home/oops/code/LOG_sayankr007-cyber_hatev1.txt`
loaded 14294 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 7 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 1000, n_clauses: 100, vote_threshold: 60, states_per_action: 120, specificity: 3.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 6, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     12.45s
------------------------------------------------------------
  Accuracy (@ V > 0): 77.19%
  Best-F1 Threshold:  V > 1
  Precision:          0.7850
  Recall:             0.6966
  F1-Score:           0.7382
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1336        244         
Actual Pos (1)    388         891         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/100  always 8/100 (8 vacuous, 0 specialized)  p25 15.1%  median 27.3%  p75 29.3%
  includes/clause: min 0  p25 1  median 1  p75 59  max 77  (8 clauses vacuous)
  vacuous vote offset: +8  (8 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 652 records to /home/oops/code/LOG_sayankr007-cyber_hatev1.txt
saved model artifact to /home/oops/models/model_json_sayankr007-cyber_hatev1.gmb


////

Batch Tests:

# binary-detect-hate

cargo run --release -- --mode batch \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto \
  --fire-guard 500

$ cargo run --release -- --mode batch \
  --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto \
  --fire-guard 500
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-hate-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto --fire-guard 500`
batch over 11435 train / 2859 test documents, seed 42
fire-guard arms enabled (limit 500): baseline matrix runs guard-OFF; byte-bag guard-ON rows follow as '<preset>+fireguard'

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     124.87s
------------------------------------------------------------
  Accuracy (@ V > 0): 55.26%
  Best-F1 Threshold:  V > -112
  Precision:          0.5210
  Recall:             0.9030
  F1-Score:           0.6608
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    518         1062        
Actual Pos (1)    124         1155        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 206/600  always 124/600 (124 vacuous, 0 specialized)  p25 0.0%  median 0.3%  p75 29.8%
  includes/clause: min 0  p25 1  median 1  p75 1  max 829  (124 clauses vacuous)
  vacuous vote offset: -72  (26 positive-polarity, 98 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     176.17s
------------------------------------------------------------
  Accuracy (@ V > 0): 72.30%
  Best-F1 Threshold:  V > -87
  Precision:          0.7826
  Recall:             0.7404
  F1-Score:           0.7609
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1317        263         
Actual Pos (1)    332         947         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 115/600 (115 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 182  (115 clauses vacuous)
  vacuous vote offset: -75  (20 positive-polarity, 95 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     131.34s
------------------------------------------------------------
  Accuracy (@ V > 0): 55.26%
  Best-F1 Threshold:  V > -112
  Precision:          0.5210
  Recall:             0.9030
  F1-Score:           0.6608
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    518         1062        
Actual Pos (1)    124         1155        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 206/600  always 124/600 (124 vacuous, 0 specialized)  p25 0.0%  median 0.3%  p75 29.8%
  includes/clause: min 0  p25 1  median 1  p75 1  max 829  (124 clauses vacuous)
  vacuous vote offset: -72  (26 positive-polarity, 98 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     178.21s
------------------------------------------------------------
  Accuracy (@ V > 0): 72.30%
  Best-F1 Threshold:  V > -87
  Precision:          0.7826
  Recall:             0.7404
  F1-Score:           0.7609
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1317        263         
Actual Pos (1)    332         947         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 115/600 (115 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 182  (115 clauses vacuous)
  vacuous vote offset: -75  (20 positive-polarity, 95 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     134.21s
------------------------------------------------------------
  Accuracy (@ V > 0): 55.26%
  Best-F1 Threshold:  V > -112
  Precision:          0.5210
  Recall:             0.9030
  F1-Score:           0.6608
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    518         1062        
Actual Pos (1)    124         1155        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 206/600  always 124/600 (124 vacuous, 0 specialized)  p25 0.0%  median 0.3%  p75 29.8%
  includes/clause: min 0  p25 1  median 1  p75 1  max 829  (124 clauses vacuous)
  vacuous vote offset: -72  (26 positive-polarity, 98 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     180.29s
------------------------------------------------------------
  Accuracy (@ V > 0): 72.30%
  Best-F1 Threshold:  V > -87
  Precision:          0.7826
  Recall:             0.7404
  F1-Score:           0.7609
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1317        263         
Actual Pos (1)    332         947         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 115/600 (115 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 182  (115 clauses vacuous)
  vacuous vote offset: -75  (20 positive-polarity, 95 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  11435/2859 samples
  Training Time:     131.74s
------------------------------------------------------------
  Accuracy (@ V > 0): 55.26%
  Best-F1 Threshold:  V > -107
  Precision:          0.5200
  Recall:             0.9054
  F1-Score:           0.6606
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    511         1069        
Actual Pos (1)    121         1158        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 225/600  always 119/600 (119 vacuous, 0 specialized)  p25 0.0%  median 0.2%  p75 29.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 732  (119 clauses vacuous)
  vacuous vote offset: -69  (25 positive-polarity, 94 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     178.35s
------------------------------------------------------------
  Accuracy (@ V > 0): 71.60%
  Best-F1 Threshold:  V > -107
  Precision:          0.7413
  Recall:             0.7819
  F1-Score:           0.7610
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1231        349         
Actual Pos (1)    279         1000        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 125/600 (125 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 180  (125 clauses vacuous)
  vacuous vote offset: -93  (16 positive-polarity, 109 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw+fireguard (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     181.03s
------------------------------------------------------------
  Accuracy (@ V > 0): 72.30%
  Best-F1 Threshold:  V > -87
  Precision:          0.7826
  Recall:             0.7404
  F1-Score:           0.7609
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1317        263         
Actual Pos (1)    332         947         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 115/600 (115 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 182  (115 clauses vacuous)
  vacuous vote offset: -75  (20 positive-polarity, 95 negative-polarity vacuous)
  fire-guard: limit 500, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0+fireguard (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     180.22s
------------------------------------------------------------
  Accuracy (@ V > 0): 72.30%
  Best-F1 Threshold:  V > -87
  Precision:          0.7826
  Recall:             0.7404
  F1-Score:           0.7609
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1317        263         
Actual Pos (1)    332         947         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 115/600 (115 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 182  (115 clauses vacuous)
  vacuous vote offset: -75  (20 positive-polarity, 95 negative-polarity vacuous)
  fire-guard: limit 500, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1+fireguard (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     177.33s
------------------------------------------------------------
  Accuracy (@ V > 0): 72.30%
  Best-F1 Threshold:  V > -87
  Precision:          0.7826
  Recall:             0.7404
  F1-Score:           0.7609
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1317        263         
Actual Pos (1)    332         947         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 115/600 (115 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 182  (115 clauses vacuous)
  vacuous vote offset: -75  (20 positive-polarity, 95 negative-polarity vacuous)
  fire-guard: limit 500, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2+fireguard (Engine: byte-bag)
  Train/Test Split:  11435/2859 samples
  Training Time:     179.64s
------------------------------------------------------------
  Accuracy (@ V > 0): 71.60%
  Best-F1 Threshold:  V > -107
  Precision:          0.7413
  Recall:             0.7819
  F1-Score:           0.7610
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    1231        349         
Actual Pos (1)    279         1000        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2859 test docs: never 0/600  always 125/600 (125 vacuous, 0 specialized)  p25 7.0%  median 15.1%  p75 25.5%
  includes/clause: min 0  p25 1  median 1  p75 1  max 180  (125 clauses vacuous)
  vacuous vote offset: -93  (16 positive-polarity, 109 negative-polarity vacuous)
  fire-guard: limit 500, resets 0
============================================================

batch total duration: 00:32:35

////
# binary-detect-offensive

cargo run --release -- --mode batch \
--data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
--clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
--specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
--epochs 12 --seed 42 --workers auto \
--fire-guard 2000
$ cargo run --release -- --mode batch \
--data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl \
--clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
--specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
--epochs 12 --seed 42 --workers auto \
--fire-guard 2000
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/NLP/language_hygeine_datasets/MULTICLASS/sayankr007-cyber-bullying-data-for-multi-label-classification-archive/binary-detect-offensive-sayankr007_cyberbullyingfinal_hateXplain_dedupe.jsonl --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto --fire-guard 2000`
batch over 10895 train / 2724 test documents, seed 42
fire-guard arms enabled (limit 2000): baseline matrix runs guard-OFF; byte-bag guard-ON rows follow as '<preset>+fireguard'

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  10895/2724 samples
  Training Time:     122.51s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.27%
  Best-F1 Threshold:  V > -77
  Precision:          0.4550
  Recall:             0.9588
  F1-Score:           0.6171
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    223         1337        
Actual Pos (1)    48          1116        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 247/600  always 125/600 (125 vacuous, 0 specialized)  p25 0.0%  median 0.1%  p75 30.2%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1220  (125 clauses vacuous)
  vacuous vote offset: -37  (44 positive-polarity, 81 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     183.90s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.12%
  Best-F1 Threshold:  V > -26
  Precision:          0.5123
  Recall:             0.8574
  F1-Score:           0.6414
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         950         
Actual Pos (1)    166         998         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 138/600 (138 vacuous, 0 specialized)  p25 2.1%  median 8.1%  p75 30.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 145  (138 clauses vacuous)
  vacuous vote offset: -4  (67 positive-polarity, 71 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  10895/2724 samples
  Training Time:     122.95s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.27%
  Best-F1 Threshold:  V > -77
  Precision:          0.4550
  Recall:             0.9588
  F1-Score:           0.6171
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    223         1337        
Actual Pos (1)    48          1116        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 247/600  always 125/600 (125 vacuous, 0 specialized)  p25 0.0%  median 0.1%  p75 30.2%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1220  (125 clauses vacuous)
  vacuous vote offset: -37  (44 positive-polarity, 81 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     184.73s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.12%
  Best-F1 Threshold:  V > -26
  Precision:          0.5123
  Recall:             0.8574
  F1-Score:           0.6414
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         950         
Actual Pos (1)    166         998         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 138/600 (138 vacuous, 0 specialized)  p25 2.1%  median 8.1%  p75 30.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 145  (138 clauses vacuous)
  vacuous vote offset: -4  (67 positive-polarity, 71 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  10895/2724 samples
  Training Time:     125.88s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.27%
  Best-F1 Threshold:  V > -77
  Precision:          0.4550
  Recall:             0.9588
  F1-Score:           0.6171
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    223         1337        
Actual Pos (1)    48          1116        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 247/600  always 125/600 (125 vacuous, 0 specialized)  p25 0.0%  median 0.1%  p75 30.2%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1220  (125 clauses vacuous)
  vacuous vote offset: -37  (44 positive-polarity, 81 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     187.71s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.12%
  Best-F1 Threshold:  V > -26
  Precision:          0.5123
  Recall:             0.8574
  F1-Score:           0.6414
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         950         
Actual Pos (1)    166         998         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 138/600 (138 vacuous, 0 specialized)  p25 2.1%  median 8.1%  p75 30.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 145  (138 clauses vacuous)
  vacuous vote offset: -4  (67 positive-polarity, 71 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  10895/2724 samples
  Training Time:     123.70s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.27%
  Best-F1 Threshold:  V > -68
  Precision:          0.4666
  Recall:             0.9115
  F1-Score:           0.6172
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    347         1213        
Actual Pos (1)    103         1061        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 250/600  always 122/600 (122 vacuous, 0 specialized)  p25 0.0%  median 0.1%  p75 21.7%
  includes/clause: min 0  p25 1  median 1  p75 1  max 1257  (122 clauses vacuous)
  vacuous vote offset: -36  (43 positive-polarity, 79 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     185.98s
------------------------------------------------------------
  Accuracy (@ V > 0): 64.94%
  Best-F1 Threshold:  V > -27
  Precision:          0.5223
  Recall:             0.8256
  F1-Score:           0.6398
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    681         879         
Actual Pos (1)    203         961         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 121/600 (121 vacuous, 0 specialized)  p25 1.7%  median 5.8%  p75 25.3%
  includes/clause: min 0  p25 1  median 1  p75 1  max 153  (121 clauses vacuous)
  vacuous vote offset: -7  (57 positive-polarity, 64 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw+fireguard (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     187.56s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.12%
  Best-F1 Threshold:  V > -26
  Precision:          0.5123
  Recall:             0.8574
  F1-Score:           0.6414
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         950         
Actual Pos (1)    166         998         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 138/600 (138 vacuous, 0 specialized)  p25 2.1%  median 8.1%  p75 30.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 145  (138 clauses vacuous)
  vacuous vote offset: -4  (67 positive-polarity, 71 negative-polarity vacuous)
  fire-guard: limit 2000, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0+fireguard (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     186.25s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.12%
  Best-F1 Threshold:  V > -26
  Precision:          0.5123
  Recall:             0.8574
  F1-Score:           0.6414
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         950         
Actual Pos (1)    166         998         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 138/600 (138 vacuous, 0 specialized)  p25 2.1%  median 8.1%  p75 30.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 145  (138 clauses vacuous)
  vacuous vote offset: -4  (67 positive-polarity, 71 negative-polarity vacuous)
  fire-guard: limit 2000, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1+fireguard (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     185.57s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.12%
  Best-F1 Threshold:  V > -26
  Precision:          0.5123
  Recall:             0.8574
  F1-Score:           0.6414
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    610         950         
Actual Pos (1)    166         998         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 138/600 (138 vacuous, 0 specialized)  p25 2.1%  median 8.1%  p75 30.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 145  (138 clauses vacuous)
  vacuous vote offset: -4  (67 positive-polarity, 71 negative-polarity vacuous)
  fire-guard: limit 2000, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2+fireguard (Engine: byte-bag)
  Train/Test Split:  10895/2724 samples
  Training Time:     172.05s
------------------------------------------------------------
  Accuracy (@ V > 0): 64.94%
  Best-F1 Threshold:  V > -27
  Precision:          0.5223
  Recall:             0.8256
  F1-Score:           0.6398
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    681         879         
Actual Pos (1)    203         961         
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 2724 test docs: never 2/600  always 121/600 (121 vacuous, 0 specialized)  p25 1.7%  median 5.8%  p75 25.3%
  includes/clause: min 0  p25 1  median 1  p75 1  max 153  (121 clauses vacuous)
  vacuous vote offset: -7  (57 positive-polarity, 64 negative-polarity vacuous)
  fire-guard: limit 2000, resets 0
============================================================

batch total duration: 00:32:50


///////////
