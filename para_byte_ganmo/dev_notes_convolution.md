# Comparison 1: "It's... alive??"

#### lite e.g.
```bash
cargo run --release --   --mode train   --train /home/oops/datasets/hate-speech-detection-curated-dataset/HateSpeechDatasetBalanced_quick.jsonl   --text-col text   --label-col label   --jsonl   --model-type flat   --model-path /home/ABC/models/HateSpeechDatasetBalancedv1.json   --epochs 5   --clauses 50   --threshold 50   --specificity 5   --max-features 100
```

# TRAIN (byte-conv only)
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
(misprediction inspection log; default: <exe_dir>/logs/)

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









## Test 1: Comparing performatizing: Byte-Convolusion Alpha Lite-Train vs. Beta-MVP-"Flat-Mode" Lite & Heavy Train

```bash
$ cargo run -- --mode train --data 
/home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/byte_test_2
   Compiling byte_convolution_series_teamgames v0.1.0 (/home/ABC/code/granmo_model_nlp_classifier_rust/byte_convolution_series_teamgames)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/byte_convolution_series_teamgames --mode train --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/byte_test_2`
loaded 99879 labeled documents
resolved config: HarnessRunConfig { profile: PreprocessProfile { stage_bits: 15 }, engine_selection: ByteConv, patch_size: 5, stride: 2, bag_ngram_len: 5, bag_vocab_size: 100, n_clauses: 50, vote_threshold: 50, states_per_action: 10, specificity: 5.0, max_scan_bytes: 1024, guarded_include: false, epochs: 5, seed: 42 }
--- run: p0 [engine: byte-conv] ---
  train/test: 79903/19976   train time: 579.76s
  accuracy @ V>0:   0.7754
  best-F1 threshold 1: P 0.8377 R 0.8296 F1 0.8337  (TP 8289 FP 1606 TN 8379 FN 1702)
  fire-rate over 19976 test docs: never 17/50  always 2/50  p25 0.0%  median 1.3%  p75 12.8%
saved model artifact to /home/ABC/models/byte_test_2
```

#### Slim defaults:
```text
  bag_vocab_size: 100, // default for quick test: moderate/normal: 4000
  n_clauses: 50,       // default for quick test: moderate/normal: 100-200
  vote_threshold: 50,
  states_per_action: 10, // default for quick test: moderate/normal: 100
  specificity: 5.0,
  max_scan_bytes: 1024,
  guarded_include: false,
  epochs: 5, // default for quick test: moderate/normal: 25
```

vs.

```bash
$ cargo run --release -- \
  --mode train \
  --train /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json  \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/ABC/models/flat1_json-cyberbullying-detection-dataset_model.json  \
  --epochs 10 \
  --clauses 80 \
  --max-features 500 \
  --jsonl
   Compiling tsetlin_windowed_nlp v0.2.0 (/home/ABC/code/tetsu_tsetlin)
    Finished `release` profile [optimized] target(s) in 9.66s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json --text-col text --label-col label --model-type flat --model-path /home/ABC/models/flat1_json-cyberbullying-detection-dataset_model.json --epochs 10 --clauses 80 --max-features 500 --jsonl`
Loading training dataset from: /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json
  Total records loaded: 99990
Splitting dataset (99990 total rows) into 80% train / 20% test...
  Split: 79992 train rows, 19998 test rows
[1/3] Building vocabulary across 79992 documents...
  Active vocabulary features: 500
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (10 epochs, 4 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 19998
  Training Time:     437.22s (19 min)
  Accuracy:        79.86%
  Macro Precision: 0.8490
  Macro Recall:    0.8048
  Macro F1-Score:  0.7923
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               ethnicity/racenot_cyberbullyingreligion      gender/sexual 
ethnicity/race 2557          66            739           45            
not_cyberbullying2             7909          2075          0             
religion       0             109           3058          5             
gender/sexual  3             123           860           2447          
============================================================

Successfully saved trained model artifact to: /home/ABC/models/flat1_json-cyberbullying-detection-dataset_model.json
```
 
vs.
```bash
$ cargo run --release -- \
  --mode train \
  --train /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl \
  --text-col text \
  --label-col label \
  --jsonl \
  --model-type flat \
  --model-path /home/ABC/models/Cyber_Bully_Data_binary_class-v4-model.json \
  --epochs 28 \
  --clauses 200 \
  --threshold 90 \
  --specificity 5 \
  --max-features 4000
(/home/ABC/code/granmo_model_nlp_classifier_rust/window_nlp_tests)
    Finished `release` profile [optimized] target(s) in 9.83s

  Total records loaded: 99879
Splitting dataset (99879 total rows) into 80% train / 20% test...
  Split: 79903 train rows, 19976 test rows
[1/3] Building vocabulary across 79903 documents...
  Active vocabulary features: 4000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (28 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 19976
  Training Time:     7514.54s
  Accuracy:        92.71%
  Macro Precision: 0.9272
  Macro Recall:    0.9271
  Macro F1-Score:  0.9271
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               1             0             
1              9312          640           
0              816           9208          
============================================================

Successfully saved trained model artifact to: /home/ABC/models/Cyber_Bully_Data_binary_class-v4-model.json
```
vs.

not entirely comparable but closer:


```bash
cargo run --release --   --mode train   --train /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl   --text-col text   --label-col label   --jsonl   --model-type flat   --model-path /home/ABC/models/Cyber_Bully_Data_binary_class-v5-slim-model.json   --epochs 5   --clauses 50   --threshold 50   --specificity 5   --max-features 100

    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --text-col text --label-col label --jsonl --model-type flat --model-path /home/ABC/models/Cyber_Bully_Data_binary_class-v5-slim-model.json --epochs 5 --clauses 50 --threshold 50 --specificity 5 --max-features 100`
Loading training dataset from: /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl
  Total records loaded: 99879
Splitting dataset (99879 total rows) into 80% train / 20% test...
  Split: 79903 train rows, 19976 test rows
[1/3] Building vocabulary across 79903 documents...
  Active vocabulary features: 100
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (5 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 19976
  Training Time:     13.11s
  Accuracy:        82.05%
  Macro Precision: 0.8460
  Macro Recall:    0.8210
  Macro F1-Score:  0.8173
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               1             0             
1              9520          432           
0              3153          6871          
============================================================
```


```


...



# Analyticisms:
Unbelievably, for a wild spaghetti throw test of a highly experimentsl convolutional-NLP + Diverse-Automata-Team Granmo-Game Model
- Vanilla Rust, no third party packages
- Strict Production-Standard: 
  -- https://github.com/lineality/rust_lang_rules
  -- https://github.com/lineality/modes_and_case_handling

Alpha Byte-Convolution:
A. Is comparable to (perhaps better than) similar settings for a vanilla-flat Granmo-Game model.
B. It is decent so far:


..
  bag_vocab_size: 100, // default for quick test: moderate/normal: 4000
  n_clauses: 50,       // default for quick test: moderate/normal: 100-200
  vote_threshold: 50,
  states_per_action: 10, // default for quick test: moderate/normal: 100
  specificity: 5.0,
  max_scan_bytes: 1024,
  guarded_include: false,
  epochs: 5, // default for quick test: moderate/normal: 25

cargo run --release -- --mode batch --data /home/ABC/datasets/cyberbullying-classification/cyberbullying_tweets_j1.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12

```
# batch test 1

```bash
ABC@fedora:~/code/granmo_model_nlp_classifier_rust/byte_convolution_series_teamgames$ cargo run --release -- --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12

    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/byte_convolution_series_teamgames --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12`
batch over 79903 train / 19976 test documents, seed 42

$ cargo run --release -- --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/byte_convolution_series_teamgames --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12`
batch over 79903 train / 19976 test documents, seed 42
--- run: raw [engine: byte-conv] ---
  train/test: 79903/19976   train time: 454.82s
  accuracy @ V>0:   0.6635
  best-F1 threshold -15: P 0.8096 R 0.8629 F1 0.8354  (TP 8621 FP 2028 TN 7957 FN 1370)
  fire-rate over 19976 test docs: never 12/200  always 22/200  p25 8.0%  median 13.5%  p75 16.4%
--- run: raw [engine: byte-bag] ---
  train/test: 79903/19976   train time: 378.61s
  accuracy @ V>0:   0.9640
  best-F1 threshold -1: P 0.9843 R 0.9466 F1 0.9650  (TP 9457 FP 151 TN 9834 FN 534)
  fire-rate over 19976 test docs: never 0/200  always 2/200  p25 10.6%  median 15.8%  p75 21.3%
--- run: p0 [engine: byte-conv] ---
  train/test: 79903/19976   train time: 468.80s
  accuracy @ V>0:   0.7119
  best-F1 threshold -9: P 0.7984 R 0.8197 F1 0.8089  (TP 8190 FP 2068 TN 7917 FN 1801)
  fire-rate over 19976 test docs: never 21/200  always 21/200  p25 2.6%  median 12.8%  p75 16.5%
--- run: p0 [engine: byte-bag] ---
  train/test: 79903/19976   train time: 351.76s
  accuracy @ V>0:   0.9646
  best-F1 threshold -2: P 0.9861 R 0.9760 F1 0.9810  (TP 9751 FP 137 TN 9848 FN 240)
  fire-rate over 19976 test docs: never 0/200  always 0/200  p25 12.7%  median 17.3%  p75 19.6%
--- run: p1 [engine: byte-conv] ---
  train/test: 79903/19976   train time: 464.90s
  accuracy @ V>0:   0.8038
  best-F1 threshold -1: P 0.7979 R 0.8016 F1 0.7998  (TP 8009 FP 2028 TN 7957 FN 1982)
  fire-rate over 19976 test docs: never 17/200  always 12/200  p25 1.4%  median 12.7%  p75 15.4%
--- run: p1 [engine: byte-bag] ---
  train/test: 79903/19976   train time: 391.75s
  accuracy @ V>0:   0.8079
  best-F1 threshold 20: P 0.9728 R 0.9601 F1 0.9664  (TP 9592 FP 268 TN 9717 FN 399)
  fire-rate over 19976 test docs: never 0/200  always 20/200  p25 15.4%  median 20.1%  p75 21.5%
--- run: p2 [engine: byte-conv] ---
  train/test: 79903/19976   train time: 480.17s
  accuracy @ V>0:   0.7238
  best-F1 threshold 5: P 0.8139 R 0.7914 F1 0.8025  (TP 7907 FP 1808 TN 8177 FN 2084)
  fire-rate over 19976 test docs: never 28/200  always 14/200  p25 0.4%  median 12.7%  p75 15.4%
--- run: p2 [engine: byte-bag] ---
  train/test: 79903/19976   train time: 348.75s
  accuracy @ V>0:   0.9697
  best-F1 threshold -2: P 0.9889 R 0.9771 F1 0.9829  (TP 9762 FP 110 TN 9875 FN 229)
  fire-rate over 19976 test docs: never 0/200  always 1/200  p25 12.9%  median 17.3%  p75 19.6%

```


# batch test 2: --guarded

```bash
cargo run --release -- --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12 --guarded
```

```
$ cargo run --release -- --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12 --guarded
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/byte_convolution_series_teamgames --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12 --guarded`
batch over 79903 train / 19976 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     440.29s
------------------------------------------------------------
  Accuracy (@ V > 0): 80.06%
  Best-F1 Threshold:  V > 2
  Precision:          0.8008
  Recall:             0.8572
  F1-Score:           0.8280
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7855        2130        
Actual Pos (1)    1427        8564        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 14/200  always 9/200  p25 7.9%  median 12.1%  p75 15.6%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     366.58s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.40%
  Best-F1 Threshold:  V > -1
  Precision:          0.9843
  Recall:             0.9466
  F1-Score:           0.9650
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9834        151         
Actual Pos (1)    534         9457        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 2/200  p25 10.6%  median 15.8%  p75 21.3%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     448.11s
------------------------------------------------------------
  Accuracy (@ V > 0): 81.84%
  Best-F1 Threshold:  V > -4
  Precision:          0.8559
  Recall:             0.8067
  F1-Score:           0.8306
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8628        1357        
Actual Pos (1)    1931        8060        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 23/200  always 16/200  p25 5.5%  median 12.7%  p75 15.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     338.83s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.46%
  Best-F1 Threshold:  V > -2
  Precision:          0.9861
  Recall:             0.9760
  F1-Score:           0.9810
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9848        137         
Actual Pos (1)    240         9751        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 0/200  p25 12.7%  median 17.3%  p75 19.6%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     439.32s
------------------------------------------------------------
  Accuracy (@ V > 0): 69.22%
  Best-F1 Threshold:  V > 7
  Precision:          0.8126
  Recall:             0.8406
  F1-Score:           0.8263
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8048        1937        
Actual Pos (1)    1593        8398        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 27/200  always 0/200  p25 0.5%  median 12.4%  p75 15.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     365.13s
------------------------------------------------------------
  Accuracy (@ V > 0): 80.79%
  Best-F1 Threshold:  V > 20
  Precision:          0.9728
  Recall:             0.9601
  F1-Score:           0.9664
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9717        268         
Actual Pos (1)    399         9592        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 20/200  p25 15.4%  median 20.1%  p75 21.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     444.69s
------------------------------------------------------------
  Accuracy (@ V > 0): 75.31%
  Best-F1 Threshold:  V > 4
  Precision:          0.8077
  Recall:             0.8263
  F1-Score:           0.8169
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8020        1965        
Actual Pos (1)    1735        8256        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 27/200  always 9/200  p25 0.3%  median 12.0%  p75 15.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     324.39s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.97%
  Best-F1 Threshold:  V > -2
  Precision:          0.9889
  Recall:             0.9771
  F1-Score:           0.9829
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9875        110         
Actual Pos (1)    229         9762        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 1/200  p25 12.9%  median 17.3%  p75 19.6%
============================================================

```

# New V2 Parallel Tests
- expectedly faster times
- unexpectedly higher %...

```
no-guard
```
$ cargo run --release -- --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12`
batch over 79903 train / 19976 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     372.90s
------------------------------------------------------------
  Accuracy (@ V > 0): 81.33%
  Best-F1 Threshold:  V > -5
  Precision:          0.8172
  Recall:             0.8549
  F1-Score:           0.8356
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8074        1911        
Actual Pos (1)    1450        8541        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 16/200  always 16/200  p25 7.9%  median 12.1%  p75 15.9%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     219.44s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.98%
  Best-F1 Threshold:  V > 0
  Precision:          0.9837
  Recall:             0.9554
  F1-Score:           0.9693
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9827        158         
Actual Pos (1)    446         9545        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 1/200  p25 12.9%  median 18.9%  p75 21.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     367.72s
------------------------------------------------------------
  Accuracy (@ V > 0): 80.15%
  Best-F1 Threshold:  V > -1
  Precision:          0.8021
  Recall:             0.7955
  F1-Score:           0.7988
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8024        1961        
Actual Pos (1)    2043        7948        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 35/200  always 17/200  p25 0.6%  median 12.6%  p75 15.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     205.93s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.84%
  Best-F1 Threshold:  V > -1
  Precision:          0.9887
  Recall:             0.9733
  F1-Score:           0.9809
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9874        111         
Actual Pos (1)    267         9724        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 2/200  p25 12.9%  median 17.3%  p75 20.2%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     368.61s
------------------------------------------------------------
  Accuracy (@ V > 0): 78.64%
  Best-F1 Threshold:  V > 2
  Precision:          0.8094
  Recall:             0.8069
  F1-Score:           0.8082
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8087        1898        
Actual Pos (1)    1929        8062        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 26/200  always 11/200  p25 0.8%  median 12.1%  p75 15.6%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     212.57s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.44%
  Best-F1 Threshold:  V > -1
  Precision:          0.9878
  Recall:             0.9512
  F1-Score:           0.9691
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9868        117         
Actual Pos (1)    488         9503        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 1/200  p25 12.9%  median 19.2%  p75 20.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     369.40s
------------------------------------------------------------
  Accuracy (@ V > 0): 80.23%
  Best-F1 Threshold:  V > -2
  Precision:          0.8145
  Recall:             0.7909
  F1-Score:           0.8025
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8185        1800        
Actual Pos (1)    2089        7902        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 40/200  always 15/200  p25 0.3%  median 12.5%  p75 15.9%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     207.09s
------------------------------------------------------------
  Accuracy (@ V > 0): 92.98%
  Best-F1 Threshold:  V > -3
  Precision:          0.9874
  Recall:             0.9751
  F1-Score:           0.9812
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9861        124         
Actual Pos (1)    249         9742        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 0/200  p25 12.9%  median 17.3%  p75 20.6%
============================================================

```



OK, and with the parallel version, somehow byte-flat did better:
```
$ cargo run --release -- --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12 --guarded
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/byte_convolution_series_teamgames --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12 --guarded`
batch over 79903 train / 19976 test documents, seed 42
   Compiling para_byte_ganmo v0.1.0 (/home/ABC/code/granmo_model_nlp_classifier_rust/para_byte_ganmo)
    Finished `release` profile [optimized] target(s) in 1.59s
     Running `target/release/para_byte_ganmo --mode batch --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12 --guarded`
batch over 79903 train / 19976 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     359.72s
------------------------------------------------------------
  Accuracy (@ V > 0): 80.57%
  Best-F1 Threshold:  V > 1
  Precision:          0.7907
  Recall:             0.8560
  F1-Score:           0.8220
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7721        2264        
Actual Pos (1)    1439        8552        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 19/200  always 9/200  p25 7.7%  median 12.7%  p75 16.0%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     212.00s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.98%
  Best-F1 Threshold:  V > 0
  Precision:          0.9837
  Recall:             0.9554
  F1-Score:           0.9693
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9827        158         
Actual Pos (1)    446         9545        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 1/200  p25 12.9%  median 18.9%  p75 21.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     364.95s
------------------------------------------------------------
  Accuracy (@ V > 0): 81.00%
  Best-F1 Threshold:  V > -1
  Precision:          0.8194
  Recall:             0.7843
  F1-Score:           0.8015
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8258        1727        
Actual Pos (1)    2155        7836        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 30/200  always 15/200  p25 1.5%  median 12.7%  p75 15.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     209.97s
------------------------------------------------------------
  Accuracy (@ V > 0): 97.84%
  Best-F1 Threshold:  V > -1
  Precision:          0.9887
  Recall:             0.9733
  F1-Score:           0.9809
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9874        111         
Actual Pos (1)    267         9724        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 2/200  p25 12.9%  median 17.3%  p75 20.2%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     373.45s
------------------------------------------------------------
  Accuracy (@ V > 0): 82.45%
  Best-F1 Threshold:  V > -3
  Precision:          0.8407
  Recall:             0.8161
  F1-Score:           0.8282
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    8440        1545        
Actual Pos (1)    1837        8154        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 24/200  always 11/200  p25 3.1%  median 13.2%  p75 15.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     213.00s
------------------------------------------------------------
  Accuracy (@ V > 0): 96.44%
  Best-F1 Threshold:  V > -1
  Precision:          0.9878
  Recall:             0.9512
  F1-Score:           0.9691
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9868        117         
Actual Pos (1)    488         9503        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 1/200  p25 12.9%  median 19.2%  p75 20.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     369.37s
------------------------------------------------------------
  Accuracy (@ V > 0): 79.44%
  Best-F1 Threshold:  V > -2
  Precision:          0.7678
  Recall:             0.8219
  F1-Score:           0.7940
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7502        2483        
Actual Pos (1)    1779        8212        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 29/200  always 9/200  p25 0.5%  median 12.4%  p75 15.4%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     202.81s
------------------------------------------------------------
  Accuracy (@ V > 0): 92.98%
  Best-F1 Threshold:  V > -3
  Precision:          0.9874
  Recall:             0.9751
  F1-Score:           0.9812
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9861        124         
Actual Pos (1)    249         9742        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/200  always 0/200  p25 12.9%  median 17.3%  p75 20.6%
============================================================


maybe better setting for conv...

```bash
cargo run --release -- \
  --mode batch \
  --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl \
  --model-out /home/oops/models/batchcyber_conv_v3.gmb \
  --clauses 600 \
  --vote-threshold 160 \
  --stride 1 \
  --patch 5 \
  --specificity 3.0 \
  --states 200 \
  --vocab-size 4000 \
  --ngram-len 5 \
  --epochs 12 \
  --guarded \
  --seed 42 \
  --workers auto
```
