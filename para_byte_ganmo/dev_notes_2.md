## TODO:
organize clean-enough datasets
- multiclass
- binary-class
-



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
--workers auto
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
$ cargo run --release -- \
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
cargo run --release --   --mode batch   --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v3.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto
```


cargo run --release --   --mode batch   --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v4.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto --guarded
