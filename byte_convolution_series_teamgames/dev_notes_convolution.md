# Comparison 1: "It's... alive??"

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
  Training Time:     437.22s
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
# batch test

```
$ cargo run --release -- --mode batch --data /home/ABC/datasets/cyberbullying-classification/cyberbullying_tweets_j1.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12
warning: unused variable: `line_index`
    --> src/main.rs:3545:10
     |
3545 |     for (line_index, line) in raw.split(|&b| b == b'\n').enumerate() {
     |          ^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_line_index`
     |
     = note: `#[warn(unused_variables)]` (part of `#[warn(unused)]`) on by default

warning: `byte_convolution_series_teamgames` (bin "byte_convolution_series_teamgames") generated 1 warning (run `cargo fix --bin "byte_convolution_series_teamgames" -p byte_convolution_series_teamgames` to apply 1 suggestion)
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/byte_convolution_series_teamgames --mode batch --data /home/ABC/datasets/cyberbullying-classification/cyberbullying_tweets_j1.jsonl --model-out /home/ABC/models/trash2 --clauses 200 --states 100 --specificity 5.0 --vocab-size 2000 --epochs 12`
batch over 38088 train / 9523 test documents, seed 42
--- run: raw [engine: byte-conv] ---
  train/test: 38088/9523   train time: 46.06s
  accuracy @ V>0:   1.0000
  best-F1 threshold -101: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 83.6%
--- run: raw [engine: byte-bag] ---
  train/test: 38088/9523   train time: 96.92s
  accuracy @ V>0:   1.0000
  best-F1 threshold -101: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 43.4%
--- run: p0 [engine: byte-conv] ---
  train/test: 38088/9523   train time: 76.71s
  accuracy @ V>0:   1.0000
  best-F1 threshold -100: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 43.6%
--- run: p0 [engine: byte-bag] ---
  train/test: 38088/9523   train time: 98.09s
  accuracy @ V>0:   1.0000
  best-F1 threshold -101: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 42.6%
--- run: p1 [engine: byte-conv] ---
  train/test: 38088/9523   train time: 47.33s
  accuracy @ V>0:   1.0000
  best-F1 threshold -101: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 81.7%
--- run: p1 [engine: byte-bag] ---
  train/test: 38088/9523   train time: 98.92s
  accuracy @ V>0:   1.0000
  best-F1 threshold -101: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 44.0%
--- run: p2 [engine: byte-conv] ---
  train/test: 38088/9523   train time: 78.12s
  accuracy @ V>0:   1.0000
  best-F1 threshold -100: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 41.8%
--- run: p2 [engine: byte-bag] ---
  train/test: 38088/9523   train time: 103.06s
  accuracy @ V>0:   1.0000
  best-F1 threshold -101: P 0.0000 R 0.0000 F1 0.0000  (TP 0 FP 9523 TN 0 FN 0)
  fire-rate over 9523 test docs: never 100/200  always 0/200  p25 0.0%  median 0.0%  p75 42.1%

```
