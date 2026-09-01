## TODO:
organize clean-enough datasets
- multiclass
- binary-class
-

quick test form
```bash
cargo run --release -- --mode batch-guard \
  --data /abs/path/XYZ.jsonl \
  --preset p0 --clauses 200 --vote-threshold 80 \
  --states 50 --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --guard-limits 0,200,1000 --test-cap 3000

```

1
```bash
cargo run --release -- --mode batch-guard \
  --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --preset p0 --clauses 200 --vote-threshold 80 \
  --states 50 --specificity 3.0 --vocab-size 1000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --guard-limits 0,200,1000 --test-cap 3000

```

2
```bash
cargo run --release -- --mode batch-guard \
  --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --preset p0 \
  --clauses 600 --vote-threshold 160 --states 200 \
  --specificity 3.0 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto --guard-limits 0
  ```



# Comparisons 2: "It's alive?"

#### lite e.g.

# TRAIN (byte-conv only)
(--log-out  misprediction inspection log; default: <exe_dir>/logs/)

```bash
cargo run --release -- \
--mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl \
--text-key text  --label-key label  --positive-label 1 \
--preset p3  --engine byte-conv \
--patch 5  --stride 2  --guarded \
--clauses 120  --vote-threshold 50  --states 100 \
--specificity 5.0  --max-scan 1024  --epochs 5  --seed 42 \
--train-percent 80  --model-out /home/oops/models/HateSpeechDatasetBalanced_quickbyte_conv_2.json \
--log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_conv_logout.txt \
--workers auto
```

# TRAIN (byte-bag only)
```bash
cargo run --release -- \
--mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl \
--text-key text  --label-key label  --positive-label 1 \
--preset p3  --engine byte-bag \
--ngram-len 5  --vocab-size 4000 \
--clauses 120  --vote-threshold 50  --states 100 \
--specificity 5.0  --max-scan 1024  --epochs 5  --seed 42 \
--train-percent 80  --model-out /home/oops/models/HateSpeechDatasetBalanced_quickbyte_bag_1.json \
--log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_bag_logout.txt \
--workers auto
```
///////////////////////////////////////////////////

# prospective

```bash
cargo run --release -- \
  --mode train \
  --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_deduplicatedv2.jsonl \
  --text-key text \
  --label-key label \
  --positive-label 1 \
  --preset p0 \
  --engine byte-bag \
  --ngram-len 5 \
  --vocab-size 16000 \
  --clauses 600 \
  --vote-threshold 200 \
  --states 400 \
  --specificity 10.0 \
  --max-scan 1024 \
  --epochs 6 \
  --seed 42 \
  --train-percent 80 \
  --model-out /home/oops/models/HateSpeech_bytebag_v3.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/HateSpeech_bytebag_v2.txt \
  --workers auto
```




///////////////////


# byte-conv 1
s
```bash
$ cargo run --release -- \
--mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl \
--text-key text  --label-key label  --positive-label 1 \
--preset p3  --engine byte-conv \
--patch 5  --stride 2  --guarded \
--clauses 120  --vote-threshold 50  --states 100 \
--specificity 5.0  --max-scan 1024  --epochs 5  --seed 42 \
--train-percent 80  --model-out /home/oops/models/byte-conv_1.json \
--log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_conv_logout.txt \
--workers auto --guarded

   Compiling para_byte_ganmo v0.1.0 (/home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo)
    Finished `release` profile [optimized] target(s) in 5.89s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl --text-key text --label-key label --positive-label 1 --preset p3 --engine byte-conv --patch 5 --stride 2 --guarded --clauses 120 --vote-threshold 50 --states 100 --specificity 5.0 --max-scan 1024 --epochs 5 --seed 42 --train-percent 80 --model-out /home/oops/models/byte-conv_1.json --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_conv_logout.txt --workers auto`
loaded 701073 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 95 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 100, n_clauses: 120, vote_threshold: 50, states_per_action: 100, specificity: 5.0, max_scan_bytes: 1024, guarded_include: true, epochs: 5, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p3           (Engine: byte-conv)
  Train/Test Split:  560858/140215 samples
  Training Time:     1204.43s
------------------------------------------------------------
  Accuracy (@ V > 0): 51.35%
  Best-F1 Threshold:  V > 18
  Precision:          0.5662
  Recall:             0.9340
  F1-Score:           0.7050
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    16759       51495       
Actual Pos (1)    4746        67215       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 140215 test docs: never 52/120  always 24/120  p25 0.0%  median 0.0%  p75 17.6%
============================================================

misprediction log: appended 68208 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_conv_logout.txt
saved model artifact to /home/oops/models/byte-conv_1.json
```


# byte-bag 1
```bash
cargo run --release -- \
--mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl \
--text-key text  --label-key label  --positive-label 1 \
--preset p3  --engine byte-bag \
--ngram-len 5  --vocab-size 4000 \
--clauses 120  --vote-threshold 50  --states 100 \
--specificity 5.0  --max-scan 1024  --epochs 5  --seed 42 \
--train-percent 80  --model-out /home/oops/models/HateSpeechDatasetBalanced_quickbyte_bag_1.json \
--log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_bag_logout.txt \
--workers auto
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl --text-key text --label-key label --positive-label 1 --preset p3 --engine byte-bag --ngram-len 5 --vocab-size 4000 --clauses 120 --vote-threshold 50 --states 100 --specificity 5.0 --max-scan 1024 --epochs 5 --seed 42 --train-percent 80 --model-out /home/oops/models/HateSpeechDatasetBalanced_quickbyte_bag_1.json --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_bag_logout.txt --workers auto`
loaded 701073 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 95 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 4000, n_clauses: 120, vote_threshold: 50, states_per_action: 100, specificity: 5.0, max_scan_bytes: 1024, guarded_include: false, epochs: 5, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p3           (Engine: byte-bag)
  Train/Test Split:  560858/140215 samples
  Training Time:     986.68s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.62%
  Best-F1 Threshold:  V > -4
  Precision:          0.6376
  Recall:             0.8650
  F1-Score:           0.7341
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    32874       35380       
Actual Pos (1)    9715        62246       
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 140215 test docs: never 0/120  always 0/120  p25 17.5%  median 19.1%  p75 20.7%
============================================================

misprediction log: appended 48201 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/byte_bag_logout.txt
saved model artifact to /home/oops/models/HateSpeechDatasetBalanced_quickbyte_bag_1.json
```


# byte-bag 2
better cleaned dataset
```bash
cargo run --release -- \
--mode train --data /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_deduplicatedv2.jsonl \
--text-key text  --label-key label  --positive-label 1 \
--preset p3  --engine byte-bag \
--ngram-len 5  --vocab-size 4000 \
--clauses 200  --vote-threshold 80  --states 100 \
--specificity 5.0  --max-scan 1024  --epochs 8  --seed 42 \
--train-percent 80  --model-out /home/oops/models/HateSpeechDatasetBalanced_deduplicatedv2-1.json \
--log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/HateSpeechDatasetBalanced_deduplicatedv2-1.txt \
--workers auto
```


# deduplicated
```bash
cargo run --release --   --mode batch   --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v3.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto
```


cargo run --release --   --mode batch   --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v4.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto --guarded


TODO: RUn
```bash
cargo run --release -- --mode batch \
  --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto \
  --fire-guard 2000
```

```bash
$ cargo run --release -- --mode batch \
  --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto \
  --fire-guard 2000
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto --fire-guard 2000`
batch over 79788 train / 19948 test documents, seed 42
fire-guard arms enabled (limit 2000): baseline matrix runs guard-OFF; byte-bag guard-ON rows follow as '<preset>+fireguard'

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     527.36s
------------------------------------------------------------
  Accuracy (@ V > 0): 59.75%
  Best-F1 Threshold:  V > -54
  Precision:          0.7138
  Recall:             0.8274
  F1-Score:           0.7664
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    6702        3300        
Actual Pos (1)    1717        8229        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 7/600  always 103/600  p25 9.5%  median 20.1%  p75 26.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     447.92s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.17%
  Best-F1 Threshold:  V > 24
  Precision:          0.9922
  Recall:             0.9719
  F1-Score:           0.9820
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9926        76          
Actual Pos (1)    279         9667        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 48/600  p25 10.6%  median 26.3%  p75 27.4%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     517.41s
------------------------------------------------------------
  Accuracy (@ V > 0): 53.01%
  Best-F1 Threshold:  V > -124
  Precision:          0.5505
  Recall:             0.9471
  F1-Score:           0.6963
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    2310        7692        
Actual Pos (1)    526         9420        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 25/600  always 123/600  p25 4.6%  median 18.8%  p75 29.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     451.21s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.68%
  Best-F1 Threshold:  V > 22
  Precision:          0.9948
  Recall:             0.9852
  F1-Score:           0.9900
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9951        51          
Actual Pos (1)    147         9799        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 44/600  p25 12.8%  median 25.8%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     567.24s
------------------------------------------------------------
  Accuracy (@ V > 0): 60.85%
  Best-F1 Threshold:  V > -50
  Precision:          0.7123
  Recall:             0.8117
  F1-Score:           0.7588
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    6742        3260        
Actual Pos (1)    1873        8073        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 12/600  always 108/600  p25 10.1%  median 20.8%  p75 26.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     481.52s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.03%
  Best-F1 Threshold:  V > 22
  Precision:          0.9953
  Recall:             0.9673
  F1-Score:           0.9811
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9957        45          
Actual Pos (1)    325         9621        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 41/600  p25 10.6%  median 26.0%  p75 27.4%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     550.20s
------------------------------------------------------------
  Accuracy (@ V > 0): 54.98%
  Best-F1 Threshold:  V > -114
  Precision:          0.5480
  Recall:             0.9415
  F1-Score:           0.6928
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    2279        7723        
Actual Pos (1)    582         9364        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 34/600  always 124/600  p25 4.6%  median 18.6%  p75 28.9%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     439.50s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.15%
  Best-F1 Threshold:  V > 30
  Precision:          0.9970
  Recall:             0.9847
  F1-Score:           0.9908
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9973        29          
Actual Pos (1)    152         9794        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 49/600  p25 12.8%  median 25.8%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw+fireguard (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     472.84s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.17%
  Best-F1 Threshold:  V > 24
  Precision:          0.9922
  Recall:             0.9719
  F1-Score:           0.9820
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9926        76          
Actual Pos (1)    279         9667        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 48/600  p25 10.6%  median 26.3%  p75 27.4%
============================================================

^[[C
============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0+fireguard (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     435.09s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.68%
  Best-F1 Threshold:  V > 22
  Precision:          0.9948
  Recall:             0.9852
  F1-Score:           0.9900
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9951        51          
Actual Pos (1)    147         9799        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 44/600  p25 12.8%  median 25.8%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1+fireguard (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     467.96s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.03%
  Best-F1 Threshold:  V > 22
  Precision:          0.9953
  Recall:             0.9673
  F1-Score:           0.9811
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9957        45          
Actual Pos (1)    325         9621        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 41/600  p25 10.6%  median 26.0%  p75 27.4%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2+fireguard (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     444.45s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.15%
  Best-F1 Threshold:  V > 30
  Precision:          0.9970
  Recall:             0.9847
  F1-Score:           0.9908
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9973        29          
Actual Pos (1)    152         9794        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 0/600  always 49/600  p25 12.8%  median 25.8%  p75 27.1%
============================================================


```


multiclass test:
# byte-bag 1
```bash
cargo run --release -- \
--mode train --data /home/oops/datasets/NLP/multiclass_sets/toxic_comments_archive/multiclass_toxic_comments_50000_onlytoxic_v1.jsonl \
--text-key text  --label-key label  --positive-label 1 \
--preset p3  --engine byte-bag \
--ngram-len 5  --vocab-size 4000 \
--clauses 120  --vote-threshold 50  --states 100 \
--specificity 5.0  --max-scan 1024  --epochs 2  --seed 42 \
--train-percent 80  --model-out /home/oops/models/multiclass_toxic_comments_50000_onlytoxic_v1-1.json \
--log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/multiclass_toxic_comments_50000_onlytoxic_v1.txt \
--workers auto
```



$ cargo run --release -- --mode train   --data  /home/oops/datasets/NLP/mental_health_datasets/BINARY_CLASS/reihanenamdari-mental-health-corpus-archive/mental_health_dedupe_v1.jsonl   --preset p0   --clauses 100 --vote-threshold 60 --states 200   --specificity 3.0 --vocab-size 1000 --ngram-len 5   --epochs 10 --seed 42 --workers auto   --train-percent 80   --model-out /home/oops/models/model_json_reihanenamdari-mental-health_v1.gmb   --log-out /home/oops/code/LOG_reihanenamdari-mental-health_v1.txt
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



...


IMDB sentiment

```bash
cargo run --release -- --mode batch \
  --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --fire-guard 1000
```

```bash
$ cargo run --release -- --mode batch \
  --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 4 --seed 42 --workers auto \
  --fire-guard 1000
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 4 --seed 42 --workers auto --fire-guard 1000`
batch over 39656 train / 9914 test documents, seed 42
fire-guard arms enabled (limit 1000): baseline matrix runs guard-OFF; byte-bag guard-ON rows follow as '<preset>+fireguard'

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  39656/9914 samples
  Training Time:     145.46s
------------------------------------------------------------
  Accuracy (@ V > 0): 53.78%
  Best-F1 Threshold:  V > -69
  Precision:          0.5043
  Recall:             0.9875
  F1-Score:           0.6677
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    142         4813        
Actual Pos (1)    62          4897        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 165/600  always 121/600 (121 vacuous, 0 specialized)  p25 0.0%  median 11.3%  p75 31.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 291  (121 clauses vacuous)
  vacuous vote offset: -35  (43 positive-polarity, 78 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     131.60s
------------------------------------------------------------
  Accuracy (@ V > 0): 71.62%
  Best-F1 Threshold:  V > -41
  Precision:          0.7620
  Recall:             0.8298
  F1-Score:           0.7945
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3670        1285        
Actual Pos (1)    844         4115        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 93/600 (93 vacuous, 0 specialized)  p25 8.1%  median 14.7%  p75 25.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 48  (93 clauses vacuous)
  vacuous vote offset: -23  (35 positive-polarity, 58 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  39656/9914 samples
  Training Time:     140.24s
------------------------------------------------------------
  Accuracy (@ V > 0): 50.11%
  Best-F1 Threshold:  V > -87
  Precision:          0.5004
  Recall:             0.9996
  F1-Score:           0.6669
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    5           4950        
Actual Pos (1)    2           4957        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 194/600  always 131/600 (130 vacuous, 1 specialized)  p25 0.0%  median 4.0%  p75 25.9%
  includes/clause: min 0  p25 1  median 1  p75 1  max 461  (130 clauses vacuous)
  vacuous vote offset: -54  (38 positive-polarity, 92 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     133.25s
------------------------------------------------------------
  Accuracy (@ V > 0): 74.43%
  Best-F1 Threshold:  V > -33
  Precision:          0.7645
  Recall:             0.8300
  F1-Score:           0.7959
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3687        1268        
Actual Pos (1)    843         4116        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 96/600 (96 vacuous, 0 specialized)  p25 8.3%  median 14.9%  p75 25.3%
  includes/clause: min 0  p25 1  median 1  p75 1  max 46  (96 clauses vacuous)
  vacuous vote offset: -12  (42 positive-polarity, 54 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  39656/9914 samples
  Training Time:     140.77s
------------------------------------------------------------
  Accuracy (@ V > 0): 56.47%
  Best-F1 Threshold:  V > -59
  Precision:          0.5023
  Recall:             0.9946
  F1-Score:           0.6675
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    68          4887        
Actual Pos (1)    27          4932        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 165/600  always 115/600 (115 vacuous, 0 specialized)  p25 0.0%  median 9.0%  p75 30.9%
  includes/clause: min 0  p25 1  median 1  p75 1  max 429  (115 clauses vacuous)
  vacuous vote offset: -19  (48 positive-polarity, 67 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     131.35s
------------------------------------------------------------
  Accuracy (@ V > 0): 77.59%
  Best-F1 Threshold:  V > -23
  Precision:          0.7452
  Recall:             0.8475
  F1-Score:           0.7931
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3518        1437        
Actual Pos (1)    756         4203        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 90/600 (90 vacuous, 0 specialized)  p25 8.1%  median 14.2%  p75 24.9%
  includes/clause: min 0  p25 1  median 1  p75 1  max 52  (90 clauses vacuous)
  vacuous vote offset: +0  (45 positive-polarity, 45 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  39656/9914 samples
  Training Time:     139.30s
------------------------------------------------------------
  Accuracy (@ V > 0): 50.26%
  Best-F1 Threshold:  V > -80
  Precision:          0.5005
  Recall:             0.9992
  F1-Score:           0.6669
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9           4946        
Actual Pos (1)    4           4955        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 218/600  always 127/600 (127 vacuous, 0 specialized)  p25 0.0%  median 0.3%  p75 22.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 344  (127 clauses vacuous)
  vacuous vote offset: -49  (39 positive-polarity, 88 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     131.89s
------------------------------------------------------------
  Accuracy (@ V > 0): 73.65%
  Best-F1 Threshold:  V > -42
  Precision:          0.7374
  Recall:             0.8625
  F1-Score:           0.7951
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3432        1523        
Actual Pos (1)    682         4277        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 84/600 (84 vacuous, 0 specialized)  p25 8.3%  median 14.7%  p75 25.3%
  includes/clause: min 0  p25 1  median 1  p75 1  max 48  (84 clauses vacuous)
  vacuous vote offset: -16  (34 positive-polarity, 50 negative-polarity vacuous)
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw+fireguard (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     132.46s
------------------------------------------------------------
  Accuracy (@ V > 0): 71.62%
  Best-F1 Threshold:  V > -41
  Precision:          0.7620
  Recall:             0.8298
  F1-Score:           0.7945
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3670        1285        
Actual Pos (1)    844         4115        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 93/600 (93 vacuous, 0 specialized)  p25 8.1%  median 14.7%  p75 25.0%
  includes/clause: min 0  p25 1  median 1  p75 1  max 48  (93 clauses vacuous)
  vacuous vote offset: -23  (35 positive-polarity, 58 negative-polarity vacuous)
  fire-guard: limit 1000, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0+fireguard (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     131.13s
------------------------------------------------------------
  Accuracy (@ V > 0): 74.43%
  Best-F1 Threshold:  V > -33
  Precision:          0.7645
  Recall:             0.8300
  F1-Score:           0.7959
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3687        1268        
Actual Pos (1)    843         4116        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 96/600 (96 vacuous, 0 specialized)  p25 8.3%  median 14.9%  p75 25.3%
  includes/clause: min 0  p25 1  median 1  p75 1  max 46  (96 clauses vacuous)
  vacuous vote offset: -12  (42 positive-polarity, 54 negative-polarity vacuous)
  fire-guard: limit 1000, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1+fireguard (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     141.09s
------------------------------------------------------------
  Accuracy (@ V > 0): 77.59%
  Best-F1 Threshold:  V > -23
  Precision:          0.7452
  Recall:             0.8475
  F1-Score:           0.7931
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3518        1437        
Actual Pos (1)    756         4203        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 90/600 (90 vacuous, 0 specialized)  p25 8.1%  median 14.2%  p75 24.9%
  includes/clause: min 0  p25 1  median 1  p75 1  max 52  (90 clauses vacuous)
  vacuous vote offset: +0  (45 positive-polarity, 45 negative-polarity vacuous)
  fire-guard: limit 1000, resets 0
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2+fireguard (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     131.95s
------------------------------------------------------------
  Accuracy (@ V > 0): 73.65%
  Best-F1 Threshold:  V > -42
  Precision:          0.7374
  Recall:             0.8625
  F1-Score:           0.7951
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3432        1523        
Actual Pos (1)    682         4277        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 84/600 (84 vacuous, 0 specialized)  p25 8.3%  median 14.7%  p75 25.3%
  includes/clause: min 0  p25 1  median 1  p75 1  max 48  (84 clauses vacuous)
  vacuous vote offset: -16  (34 positive-polarity, 50 negative-polarity vacuous)
  fire-guard: limit 1000, resets 0
============================================================

batch total duration: 00:27:54

```

next:

cargo run --release -- --mode train \
  --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --engine byte-bag --preset p0 \
  --clauses 600 --vote-threshold 180 \
  --states 100 --specificity 4.0 \
  --vocab-size 6000 --ngram-len 4 \
  --epochs 10 --seed 42 --workers auto


```bash
cargo run --release -- --mode train \
--data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 \
  --engine byte-bag \
  --clauses 600 \
  --vote-threshold 180 \
  --states 100 \
  --specificity 4.0 \
  --vocab-size 6000 \
  --ngram-len 4 \
  --epochs 10 \
  --seed 42 \
  --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdbtest3.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
  ```

$ cargo run --release -- --mode train \
--data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset raw \
  --engine byte-bag \
  --clauses 600 \
  --vote-threshold 180 \
  --states 100 \
  --specificity 4.0 \
  --vocab-size 6000 \
  --ngram-len 4 \
  --epochs 10 \
  --seed 42 \
  --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdbtest3.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset raw --engine byte-bag --clauses 600 --vote-threshold 180 --states 100 --specificity 4.0 --vocab-size 6000 --ngram-len 4 --epochs 10 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdbtest3.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 0 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 6000, n_clauses: 600, vote_threshold: 180, states_per_action: 100, specificity: 4.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 10, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     404.15s
------------------------------------------------------------
  Accuracy (@ V > 0): 82.31%
  Best-F1 Threshold:  V > -4
  Precision:          0.8048
  Recall:             0.8463
  F1-Score:           0.8250
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3937        1018        
Actual Pos (1)    762         4197        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 3/600 (3 vacuous, 0 specialized)  p25 18.0%  median 20.1%  p75 22.1%
  includes/clause: min 0  p25 37  median 45  p75 52  max 67  (3 clauses vacuous)
  vacuous vote offset: +3  (3 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1754 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
saved model artifact to /home/oops/models/imdbtest3.gmb

$ cargo run --release -- --mode train \
--data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p0 \
  --engine byte-bag \
  --clauses 600 \
  --vote-threshold 180 \
  --states 100 \
  --specificity 4.0 \
  --vocab-size 6000 \
  --ngram-len 4 \
  --epochs 10 \
  --seed 42 \
  --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdbtest3.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p0 --engine byte-bag --clauses 600 --vote-threshold 180 --states 100 --specificity 4.0 --vocab-size 6000 --ngram-len 4 --epochs 10 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdbtest3.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 6000, n_clauses: 600, vote_threshold: 180, states_per_action: 100, specificity: 4.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 10, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     391.73s
------------------------------------------------------------
  Accuracy (@ V > 0): 81.95%
  Best-F1 Threshold:  V > -5
  Precision:          0.8016
  Recall:             0.8435
  F1-Score:           0.8220
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3920        1035        
Actual Pos (1)    776         4183        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 6/600 (6 vacuous, 0 specialized)  p25 18.2%  median 20.1%  p75 22.2%
  includes/clause: min 0  p25 36  median 43  p75 49  max 65  (6 clauses vacuous)
  vacuous vote offset: +6  (6 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1789 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
saved model artifact to /home/oops/models/imdbtest3.gmb


p1

cargo run --release -- --mode train \
--data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p1 \
  --engine byte-bag \
  --clauses 600 \
  --vote-threshold 180 \
  --states 100 \
  --specificity 4.0 \
  --vocab-size 6000 \
  --ngram-len 4 \
  --epochs 10 \
  --seed 42 \
  --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdbtest3.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt



$ cargo run --release -- --mode train \
--data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset p1 \
  --engine byte-bag \
  --clauses 600 \
  --vote-threshold 180 \
  --states 100 \
  --specificity 4.0 \
  --vocab-size 6000 \
  --ngram-len 4 \
  --epochs 10 \
  --seed 42 \
  --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdbtest3.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode train --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl --preset p1 --engine byte-bag --clauses 600 --vote-threshold 180 --states 100 --specificity 4.0 --vocab-size 6000 --ngram-len 4 --epochs 10 --seed 42 --workers auto --train-percent 80 --model-out /home/oops/models/imdbtest3.gmb --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt`
loaded 49570 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 7 }, engine_selection: ByteBag, patch_size: 5, stride: 2, bag_ngram_len: 4, bag_vocab_size: 6000, n_clauses: 600, vote_threshold: 180, states_per_action: 100, specificity: 4.0, max_scan_bytes: 1024, guarded_include: false, fire_guard_streak_limit: 0, epochs: 10, seed: 42, worker_count: 16 }

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  39656/9914 samples
  Training Time:     392.07s
------------------------------------------------------------
  Accuracy (@ V > 0): 82.29%
  Best-F1 Threshold:  V > -6
  Precision:          0.7939
  Recall:             0.8592
  F1-Score:           0.8253
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    3849        1106        
Actual Pos (1)    698         4261        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 9914 test docs: never 0/600  always 3/600 (3 vacuous, 0 specialized)  p25 17.8%  median 19.9%  p75 22.0%
  includes/clause: min 0  p25 38  median 45  p75 51  max 69  (3 clauses vacuous)
  vacuous vote offset: +3  (3 positive-polarity, 0 negative-polarity vacuous)
============================================================

misprediction log: appended 1756 records to /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
saved model artifact to /home/oops/models/imdbtest3.gmb

vs.


cargo run --release -- --mode train \
  --data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --engine byte-bag 
  --preset p0 \
  --clauses 800 
  --vote-threshold 220 \
  --states 120 
  --specificity 4.5 \
  --vocab-size 8000 
  --ngram-len 5 \
  --epochs 12 
  --seed 42 
  --workers auto

raw
```bash
cargo run --release -- --mode train \
--data /home/oops/Downloads/lakshmi25npathi-imdb-dataset-of-50k-movie-reviews-archive/IMDBDataset_dedupe_detect_negative.jsonl \
  --preset raw \
  --engine byte-bag \
  --clauses 800 \
  --vote-threshold 220 \
  --states 120 \
  --specificity 4.5 \
  --vocab-size 8000 \
  --ngram-len 5 \
  --epochs 12 \
  --seed 42 \
  --workers auto \
  --train-percent 80 \
  --model-out /home/oops/models/imdbtest_4.gmb \
  --log-out /home/oops/code/granmo_model_nlp_classifier_rust/para_byte_ganmo/logs/test.txt
  ```
