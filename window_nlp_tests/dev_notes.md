Dev Notes! Well & Less-Well Done -> Datums

```
--model-type windowed (convolutional) vs. flat (bag of words, no context)

use --jsonl (flag without other input) for json(l)

# standard fast-test
  --epochs 7 \
  --clauses 10 \
  --max-features 100

# standard big maxy-test
  --epochs 32 \
  --clauses 256 \
  --max-features 6000


# Quicks
cargo run --release -- \
  --mode train \
  --train HERE  \ 
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/HERE  \
  --epochs 10 \
  --clauses 20 \
  --threshold 30 \
  --specificity 3 \
  --max-features 100

  ```
```

possible critiques of window-experiments: (from gemini)
Extreme Literal Sparsity & Feedback Dilution: In a word-level sliding window (w=3,V=4000), each window contains exactly 3 active positive literals out of 12,000. Under Type Ia feedback, the TM attempts to reinforce the 3 present words while decaying the remaining 11,997. Because 1/sberasure is stochastic, false literals are continually erased, but the probability of a clause settling on a coherent combination across sparse word IDs drops exponentially.
Vocabulary Fragmentation: In a flat BOW, occurrences of "good" anywhere in the document contribute to the same literal. In a windowed word model, "good at slot 0", "good at slot 1", and "good at slot 2" are three distinct, decoupled literals. A clause searching for the concept "good" has its learning capacity divided across all slots.
No Sub-Word Sharing: Misspellings, inflections, and morphological affixes ("un-", "-toxic", "-ing") are treated as entirely independent dimensions in word-level vocabularies.
```


  

"    --threshold <N>        Threshold target (default: 50)"
"    --specificity <F>      Specificity parameter (default: 5.0)"
┌────────────────────────────────────────────────────────────────────────┐
│                        Tsetlin Machine Tuning Formulas                 │
├──────────────────────────┬─────────────────────────────────────────────┤
│ Target Threshold (T)     │ T ≈ 0.5 × clauses  to  0.8 × clauses        │
│                          │ For clauses = 120  ──►  T = 50..80          │
├──────────────────────────┼─────────────────────────────────────────────┤
│ Specificity (s)          │ Controls clause length (literal count):     │
│                          │ s = 3.0..5.0  ──► Short clauses (2-4 words) │
│                          │ s = 6.0..10.0 ──► Long clauses (5-8 words)  │
├──────────────────────────┼─────────────────────────────────────────────┤
│ States per action (N)    │ N = 100 (Total 200 states)                  │
│                          │ Prevents noise from flipping logic decisions│
└──────────────────────────┴─────────────────────────────────────────────┘
```
```

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/archive_hsosd/train.csv \
  --text-col tweet \
  --label-col class \
  --model-type flat \
  --model-path /home/oops/models/flat_hsosd_model.json  \
  --epochs 12 \
  --clauses 40 \
  --max-features 2000

============================================================
               Classification Evaluation Report             
============================================================
  Accuracy:        72.22%
  Macro Precision: 0.4436
  Macro Recall:    0.4175
  Macro F1-Score:  0.4128
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               2             1             0             
2              329           532           11            
1              532           3245          21            
0              40            241           6             
============================================================

//////////////////////////////////////////////////////////////////////
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english.csv \
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/win_eng-toxic-lang-dataset-nlp_model.json  \
  --epochs 15 \
  --clauses 50 \
  --max-features 2500


  ============================================================
                 Classification Evaluation Report             
  ============================================================
    Accuracy:        76.90%
    Macro Precision: 0.8417
    Macro Recall:    0.7697
    Macro F1-Score:  0.7563
  ------------------------------------------------------------
  Confusion Matrix (Rows: Actual, Columns: Predicted):
                 0             1             
  0              997           0             
  1              462           541           
  ============================================================

//////////////////////////////////////////////////////////////////////
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english.csv \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat2_eng-toxic-lang-dataset-nlp_model.json  \
  --epochs 15 \
  --clauses 50 \
  --max-features 2500

  ============================================================
                 Classification Evaluation Report             
  ============================================================
    Evaluated Samples: 2000
    Training Time:     3.41s
    Accuracy:        93.20%
    Macro Precision: 0.9400
    Macro Recall:    0.9322
    Macro F1-Score:  0.9317
  ------------------------------------------------------------
  Confusion Matrix (Rows: Actual, Columns: Predicted):
                 0             1             
  0              997           0             
  1              136           867           
  ============================================================


//////////////////////////////////////////////////////////////////////
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english.csv \
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed2_eng-toxic-lang-dataset-nlp_model.json  \
  --epochs 15 \
  --clauses 50 \
  --max-features 2500

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 2000
  Training Time:     7.80s
  Accuracy:        76.90%
  Macro Precision: 0.8417
  Macro Recall:    0.7697
  Macro F1-Score:  0.7563
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               0             1             
0              997           0             
1              462           541           
============================================================


//////////////////////////////////////////////////////////////////////


cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english.csv \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flatmax3_eng-toxic-lang-dataset-nlp_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000


============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 2000
  Training Time:     20.89s
  Accuracy:        100.00%
  Macro Precision: 1.0000
  Macro Recall:    1.0000
  Macro F1-Score:  1.0000
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               0             1             
0              997           0             
1              0             1003          
============================================================

///////////////////////////////////////////////////

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/english-toxic-language-dataset-for-nlp/toxic_comments_english.csv \
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/winmax3_eng-toxic-lang-dataset-nlp_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000


============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 2000
  Training Time:     49.35s
  Accuracy:        53.80%
  Macro Precision: 0.7595
  Macro Recall:    0.5394
  Macro F1-Score:  0.4147
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               0             1             
0              997           0             
1              924           79            
============================================================


///////////////////////////////////////////////////
$ cargo run --release --   --mode train   --train /home/oops/datasets/archive_hsosd/train.csv   --text-col tweet   --label-col class   --model-type windowed   --model-path /home/oops/models/win3_hsosd_model.json    --epochs 15   --clauses 64   --max-features 2750
   Compiling proc-macro2 v1.0.107
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/archive_hsosd/train.csv --text-col tweet --label-col class --model-type windowed --model-path /home/oops/models/win3_hsosd_model.json --epochs 15 --clauses 64 --max-features 2750`
Loading training dataset from: /home/oops/datasets/archive_hsosd/train.csv
  Total records loaded: 24783
Splitting dataset (24783 total rows) into 80% train / 20% test...
  Split: 19826 train rows, 4957 test rows
[1/3] Building vocabulary across 19826 documents...
  Active vocabulary features: 2750
[2/3] Pre-computing word-ID token sequences (windowed path)...
[3/3] Training WindowedTM (width 3, CountFire pooling, cap 3, 15 epochs, 3 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 4957
  Training Time:     879.36s
  Accuracy:        74.80%
  Macro Precision: 0.3469
  Macro Recall:    0.3446
  Macro F1-Score:  0.3243
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               2             1             0             
2              65            807           0             
1              152           3643          3             
0              26            261           0             
============================================================

Successfully saved trained model artifact to: /home/oops/models/win3_hsosd_model.json



//////////////////////////////////////////////////////////////////////
# Note Scores of This Run

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv  \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_toxic-comment-detection-dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000


$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv  \ 
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_toxic-comment-detection-dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv --text-col text --label-col label --model-type flat --model-path /home/oops/models/flat1_toxic-comment-detection-dataset_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv
  Total records loaded: 10000
Splitting dataset (10000 total rows) into 80% train / 20% test...
  Split: 8000 train rows, 2000 test rows
[1/3] Building vocabulary across 8000 documents...
  Active vocabulary features: 2683
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (50 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 2000
  Training Time:     549.84s
  Accuracy:        100.00%
  Macro Precision: 1.0000
  Macro Recall:    1.0000
  Macro F1-Score:  1.0000
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               Toxic         Non-Toxic     
Toxic          1075          0             
Non-Toxic      0             925           
============================================================

Successfully saved trained model artifact to: /home/oops/models/flat1_toxic-comment-detection-dataset_model.json


//////////////////////////////////////////////////////////////////////
# Note Scores of This Run

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv  \
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv  \ 
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv --text-col text --label-col label --model-type windowed --model-path /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv
  Total records loaded: 10000
Splitting dataset (10000 total rows) into 80% train / 20% test...
  Split: 8000 train rows, 2000 test rows
[1/3] Building vocabulary across 8000 documents...
  Active vocabulary features: 2683
[2/3] Pre-computing word-ID token sequences (windowed path)...
[3/3] Training WindowedTM (width 3, CountFire pooling, cap 3, 50 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 2000
  Training Time:     195.31s
  Accuracy:        100.00%
  Macro Precision: 1.0000
  Macro Recall:    1.0000
  Macro F1-Score:  1.0000
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               Toxic         Non-Toxic     
Toxic          1075          0             
Non-Toxic      0             925           
============================================================

Successfully saved trained model artifact to: /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json



//////////////////////////////////////////////////////////////////////

// Accuracy:        91.22%

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv  \
  --text-col comment_text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_toxic_comments_dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv  \
  --text-col comment_text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_toxic_comments_dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.07s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv --text-col comment_text --label-col label --model-type flat --model-path /home/oops/models/flat1_toxic_comments_dataset_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv
  Total records loaded: 49998
Splitting dataset (49998 total rows) into 80% train / 20% test...
  Split: 39998 train rows, 10000 test rows
[1/3] Building vocabulary across 39998 documents...
  Active vocabulary features: 3190
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (50 epochs, 6 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 10000
  Training Time:     8434.89s
  Accuracy:        91.22%
  Macro Precision: 0.9306
  Macro Recall:    0.9126
  Macro F1-Score:  0.9155
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               toxic         severe_toxic  obscene       insult        threat        hate_speech   
toxic          1593          18            0             1             0             0             
severe_toxic   157           1543          3             0             0             0             
obscene        86            27            1527          2             0             0             
insult         180           75            8             1389          0             0             
threat         158           24            0             0             1509          0             
hate_speech    98            31            7             3             0             1561          
============================================================

Successfully saved trained model artifact to: /home/oops/models/flat1_toxic_comments_dataset_model.json



//////////////////////////////////////////////////////////////////////
# Note Scores of This Run
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv  \
  --text-col comment_text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_toxic_comments_dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv  \ 
  --text-col text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv --text-col text --label-col label --model-type windowed --model-path /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/toxic-comment-detection-dataset/toxic_train.csv
  Total records loaded: 10000
Splitting dataset (10000 total rows) into 80% train / 20% test...
  Split: 8000 train rows, 2000 test rows
[1/3] Building vocabulary across 8000 documents...
  Active vocabulary features: 2683
[2/3] Pre-computing word-ID token sequences (windowed path)...
[3/3] Training WindowedTM (width 3, CountFire pooling, cap 3, 50 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 2000
  Training Time:     195.31s
  Accuracy:        100.00%
  Macro Precision: 1.0000
  Macro Recall:    1.0000
  Macro F1-Score:  1.0000
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               Toxic         Non-Toxic     
Toxic          1075          0             
Non-Toxic      0             925           
============================================================

Successfully saved trained model artifact to: /home/oops/models/windowed_1_toxic-comment-detection-dataset_model.json

//////////////////////////////////////////////////////////////////////

$ cd code/tetsu_tsetlin/
oops@fedora:~/code/tetsu_tsetlin$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv  \
  --text-col comment_text \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_toxic_comments_dataset_model.json  \ 
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv --text-col comment_text --label-col label --model-type windowed --model-path /home/oops/models/windowed_1_toxic_comments_dataset_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/toxic_comments_archive/toxic_comments_dataset.csv
  Total records loaded: 49998
Splitting dataset (49998 total rows) into 80% train / 20% test...
  Split: 39998 train rows, 10000 test rows
[1/3] Building vocabulary across 39998 documents...
  Active vocabulary features: 3190
[2/3] Pre-computing word-ID token sequences (windowed path)...
[3/3] Training WindowedTM (width 3, CountFire pooling, cap 3, 50 epochs, 6 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 10000
  Training Time:     20995.01s
  Accuracy:        95.64%
  Macro Precision: 0.9608
  Macro Recall:    0.9563
  Macro F1-Score:  0.9567
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               toxic         severe_toxic  obscene       insult        threat        hate_speech   
toxic          1594          14            4             0             0             0             
severe_toxic   83            1617          3             0             0             0             
obscene        56            14            1567          5             0             0             
insult         142           62            3             1445          0             0             
threat         26            1             0             0             1664          0             
hate_speech    20            3             0             0             0             1677          
============================================================

Successfully saved trained model artifact to: /home/oops/models/windowed_1_toxic_comments_dataset_model.json

//////////////////////////////////////////////////////////////////////

// min-run 1 

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv  \
  --text-col tweet_text \
  --label-col cyberbullying_type \
  --model-type flat \
  --model-path /home/oops/models/flat1_cyberbullying-classification_model.json  \
  --epochs 7 \
  --clauses 20 \
  --max-features 200

  
  $ cargo run --release -- \
    --mode train \
    --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv  \
    --text-col tweet_text \
    --label-col cyberbullying_type \
    --model-type flat \
    --model-path /home/oops/models/flat1_cyberbullying-classification_model.json  \
    --epochs 7 \
    --clauses 20 \
    --max-features 200
      Finished `release` profile [optimized] target(s) in 0.03s
       Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv --text-col tweet_text --label-col cyberbullying_type --model-type flat --model-path /home/oops/models/flat1_cyberbullying-classification_model.json --epochs 7 --clauses 20 --max-features 200`
  Loading training dataset from: /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv
    Total records loaded: 47692
  Splitting dataset (47692 total rows) into 80% train / 20% test...
    Split: 38154 train rows, 9538 test rows
  [1/3] Building vocabulary across 38154 documents...
    Active vocabulary features: 200
  [2/3] Pre-computing flat BOW vectors (baseline path)...
  [3/3] Training flat VanillaTM (7 epochs, 6 classes)...
  
  ============================================================
                 Classification Evaluation Report             
  ============================================================
    Evaluated Samples: 9538
    Training Time:     27.64s
    Accuracy:        57.71%
    Macro Precision: 0.5939
    Macro Recall:    0.5740
    Macro F1-Score:  0.5690
  ------------------------------------------------------------
  Confusion Matrix (Rows: Actual, Columns: Predicted):
                 not_cyberbullyinggender        religion      other_cyberbullyingage           ethnicity     
  not_cyberbullying823           217           238           138           104           50            
  gender         346           1012          69            60            13            29            
  religion       218           87            1221          49            15            27            
  other_cyberbullying557           294           295           249           80            84            
  age            361           69            29            120           967           17            
  ethnicity      231           104           48            66            19            1232          
  ============================================================

//////////////////////////////////////////////////////////////////////
$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv  \
  --text-col tweet_text \
  --label-col cyberbullying_type \
  --model-type flat \
  --model-path /home/oops/models/flat2_cyberbullying-classification_model.json  \
  --epochs 20 \
  --clauses 80 \
  --max-features 2000

  
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv --text-col tweet_text --label-col cyberbullying_type --model-type flat --model-path /home/oops/models/flat2_cyberbullying-classification_model.json --epochs 20 --clauses 80 --max-features 2000`
Loading training dataset from: /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv
  Total records loaded: 47692
Splitting dataset (47692 total rows) into 80% train / 20% test...
  Split: 38154 train rows, 9538 test rows
[1/3] Building vocabulary across 38154 documents...
  Active vocabulary features: 2000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (20 epochs, 6 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 9538
  Training Time:     2573.42s
  Accuracy:        70.38%
  Macro Precision: 0.6885
  Macro Recall:    0.7002
  Macro F1-Score:  0.6803
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               not_cyberbullyinggender        religion      other_cyberbullyingage           ethnicity     
not_cyberbullying586           143           323           282           162           74            
gender         123           1218          78            59            26            25            
religion       20            41            1527          14            11            4             
other_cyberbullying172           230           407           516           134           100           
age            94            53            27            27            1356          6             
ethnicity      46            41            59            24            20            1510          
============================================================

Successfully saved trained model artifact to: /home/oops/models/flat2_cyberbullying-classification_model.json

/////////////////////////////////////////

# note prediction emprovement when combining split-positive classes into one

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v1.jsonl \
  --text-col text \
  --label-col label \
  --jsonl \
  --model-type flat \
  --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v1-model.json \
  --epochs 16 \
  --clauses 100 \
  --threshold 50 \
  --specificity 5 \
  --max-features 1240
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v1.jsonl --text-col text --label-col label --jsonl --model-type flat --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v1-model.json --epochs 16 --clauses 100 --threshold 50 --specificity 5 --max-features 1240`
Loading training dataset from: /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v1.jsonl
  Total records loaded: 99989
Splitting dataset (99989 total rows) into 80% train / 20% test...
  Split: 79991 train rows, 19998 test rows
[1/3] Building vocabulary across 79991 documents...
  Active vocabulary features: 1240
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (16 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 19998
  Training Time:     746.52s
  Accuracy:        93.71%
  Macro Precision: 0.9376
  Macro Recall:    0.9370
  Macro F1-Score:  0.9371
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               0             1             
0              9583          466           
1              792           9157          
============================================================

Successfully saved trained model artifact to: /home/oops/models/Cyber_Bully_Data_binary_class-v1-model.json



//////////////////////////////////////////////////////////////////////

// notably bad (maybe dataset formatting issues)
// I do not see over file issues (such as others had)

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv  \
  --text-col comment \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_subhajeetdas-hate-comment_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv  \
  --text-col comment \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_subhajeetdas-hate-comment_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv --text-col comment --label-col label --model-type flat --model-path /home/oops/models/flat1_subhajeetdas-hate-comment_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/subhajeetdas-hate-comment/hate.csv
Error: CsvError { path: "/home/oops/datasets/subhajeetdas-hate-comment/hate.csv", source: Error(Utf8 { pos: Some(Position { byte: 3381402, line: 23704, record: 23602 }), err: Utf8Error { field: 1, valid_up_to: 99 } }) }
oops@fedora:~/code/tetsu_tsetlin$ cargo run --release --   --mode train   --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv    --text-col comment   --label-col label   --model-type flat   --model-path /home/oops/models/flat1_subhajeetdas-hate-comment_model.json    --epochs 50   --clauses 100   --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv --text-col comment --label-col label --model-type flat --model-path /home/oops/models/flat1_subhajeetdas-hate-comment_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/subhajeetdas-hate-comment/hate.csv
  Total records loaded: 41144
Splitting dataset (41144 total rows) into 80% train / 20% test...
  Split: 32915 train rows, 8229 test rows
[1/3] Building vocabulary across 32915 documents...
  Active vocabulary features: 4000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (50 epochs, 3 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 8229
  Training Time:     6552.39s
  Accuracy:        36.35%
  Macro Precision: 0.3933
  Macro Recall:    0.3341
  Macro F1-Score:  0.2600
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               N             P             O             
N              647           2557          1136          
P              275           2343          1267          
O              0             3             1             
============================================================

Successfully saved trained model artifact to: /home/oops/models/flat1_subhajeetdas-hate-comment_model.json

//////////////////////////////////////////////////////////////////////


cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv  \
  --text-col comment \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_subhajeetdas-hate-comment_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv  \
  --text-col comment \
  --label-col label \
  --model-type windowed \
  --model-path /home/oops/models/windowed_1_subhajeetdas-hate-comment_model.json  \
  --epochs 50 \
  --clauses 100 \
  --max-features 4000
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/subhajeetdas-hate-comment/hate.csv --text-col comment --label-col label --model-type windowed --model-path /home/oops/models/windowed_1_subhajeetdas-hate-comment_model.json --epochs 50 --clauses 100 --max-features 4000`
Loading training dataset from: /home/oops/datasets/subhajeetdas-hate-comment/hate.csv
  Total records loaded: 41144
Splitting dataset (41144 total rows) into 80% train / 20% test...
  Split: 32915 train rows, 8229 test rows
[1/3] Building vocabulary across 32915 documents...
  Active vocabulary features: 4000
[2/3] Pre-computing word-ID token sequences (windowed path)...
[3/3] Training WindowedTM (width 3, CountFire pooling, cap 3, 50 epochs, 3 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 8229
  Training Time:     12451.83s
  Accuracy:        47.21%
  Macro Precision: 0.1574
  Macro Recall:    0.3333
  Macro F1-Score:  0.2138
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               N             P             O             
N              0             4340          0             
P              0             3885          0             
O              0             4             0             
============================================================

Successfully saved trained model artifact to: /home/oops/models/windowed_1_subhajeetdas-hate-comment_model.json

//////////////////////////////////////////////////////////

//json test 1

cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json  \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_json-cyberbullying-detection-dataset_model.json  \
  --epochs 10 \
  --clauses 80 \
  --max-features 500 \
  --jsonl

$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json  \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat1_json-cyberbullying-detection-dataset_model.json  \
  --epochs 10 \
  --clauses 80 \
  --max-features 500 \
  --jsonl
   Compiling tsetlin_windowed_nlp v0.2.0 (/home/oops/code/tetsu_tsetlin)
    Finished `release` profile [optimized] target(s) in 9.66s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json --text-col text --label-col label --model-type flat --model-path /home/oops/models/flat1_json-cyberbullying-detection-dataset_model.json --epochs 10 --clauses 80 --max-features 500 --jsonl`
Loading training dataset from: /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json
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

Successfully saved trained model artifact to: /home/oops/models/flat1_json-cyberbullying-detection-dataset_model.json


//////////////////////////////////////////////////////////


cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data.json  \
  --text-col text \
  --label-col label \
  --model-type flat \
  --model-path /home/oops/models/flat2_maxy_json-cyberbullying-detection-dataset_model.json  \
  --epochs 32 \
  --clauses 200 \
  --max-features 6000 \
  --jsonl


//////////////////////////////////////////////////////////////////////
cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv  \
  --text-col tweet_text \
  --label-col cyberbullying_type \
  --model-type flat \
  --model-path /home/oops/models/flat3_maxy_cyberbullying-classification_model.json  \
  --epochs 32 \
  --clauses 200 \
  --max-features 5000


$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv  \
  --text-col tweet_text \
  --label-col cyberbullying_type \
  --model-type flat \
  --model-path /home/oops/models/flat3_maxy_cyberbullying-classification_model.json  \
  --epochs 32 \
  --clauses 200 \
  --max-features 5000
    Finished `release` profile [optimized] target(s) in 0.03s
     Running `target/release/tsetlin_windowed_nlp --mode train --train /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv --text-col tweet_text --label-col cyberbullying_type --model-type flat --model-path /home/oops/models/flat3_maxy_cyberbullying-classification_model.json --epochs 32 --clauses 200 --max-features 5000`
Loading training dataset from: /home/oops/datasets/cyberbullying-classification/cyberbullying_tweets.csv
  Total records loaded: 47692
Splitting dataset (47692 total rows) into 80% train / 20% test...
  Split: 38154 train rows, 9538 test rows
[1/3] Building vocabulary across 38154 documents...
  Active vocabulary features: 5000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (32 epochs, 6 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 9538
  Training Time:     16268.11s
  Accuracy:        72.89%
  Macro Precision: 0.8350
  Macro Recall:    0.7252
  Macro F1-Score:  0.6969
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               not_cyberbullyinggender        religion      other_cyberbullyingage           ethnicity     
not_cyberbullying1463          7             37            3             56            4             
gender         388           1134          2             1             3             1             
religion       228           15            1371          0             1             2             
other_cyberbullying1475          33            4             24            17            6             
age            135           1             2             0             1423          2             
ethnicity      152           2             2             3             4             1537          
============================================================

Successfully saved trained model artifact to: /home/oops/models/flat3_maxy_cyberbullying-classification_model.json


////////////////////////////////


File Formatting test:
After a bit of file-wrangling and making/using 
https://github.com/lineality/dataset_filechecker
to make a jsonl format that is Rust-friendly,

it may be that a clean jsonl version of the original dodgy-csv-format is performing better

note: merging into a single-class-prediction may also be good to try here



```
$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/maybe_format_bad/archive_hsosd/train_quick_jsonl_v11.jsonl \
  --text-col tweet \
  --label-col class \
  --jsonl \
  --model-type flat \
  --model-path /home/oops/models/archive_hsosd-json-2-model.json \
  --epochs 10 \
  --clauses 20 \
  --threshold 30 \
  --specificity 3 \
  --max-features 50
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/oops/datasets/maybe_format_bad/archive_hsosd/train_quick_jsonl_v11.jsonl --text-col tweet --label-col class --jsonl --model-type flat --model-path /home/oops/models/archive_hsosd-json-2-model.json --epochs 10 --clauses 20 --threshold 30 --specificity 3 --max-features 50`
Loading training dataset from: /home/oops/datasets/maybe_format_bad/archive_hsosd/train_quick_jsonl_v11.jsonl
  Total records loaded: 24783
Splitting dataset (24783 total rows) into 80% train / 20% test...
  Split: 19826 train rows, 4957 test rows
[1/3] Building vocabulary across 19826 documents...
  Active vocabulary features: 50
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (10 epochs, 3 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 4957
  Training Time:     2.72s
  Accuracy:        72.26%
  Macro Precision: 0.4166
  Macro Recall:    0.3920
  Macro F1-Score:  0.3972
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               2             1             0             
2              227           634           11            
1              332           3345          121           
0              25            252           10            
============================================================

Successfully saved trained model artifact to: /home/oops/models/archive_hsosd-json-2-model.json
oops@fedora:~/code/granmo_model_nlp_classifier_rust/window_nlp_tests$   cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/maybe_format_bad/archive_hsosd/train_quick_jsonl_v11.jsonl \
  --text-col tweet \
  --label-col class \
  --jsonl \
  --model-type flat \
  --model-path /home/oops/models/archive_hsosd-json-2-model.json \
  --epochs 12 \
  --clauses 50 \
  --threshold 40 \
  --specificity 4 \
  --max-features 1000
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/oops/datasets/maybe_format_bad/archive_hsosd/train_quick_jsonl_v11.jsonl --text-col tweet --label-col class --jsonl --model-type flat --model-path /home/oops/models/archive_hsosd-json-2-model.json --epochs 12 --clauses 50 --threshold 40 --specificity 4 --max-features 1000`
Loading training dataset from: /home/oops/datasets/maybe_format_bad/archive_hsosd/train_quick_jsonl_v11.jsonl
  Total records loaded: 24783
Splitting dataset (24783 total rows) into 80% train / 20% test...
  Split: 19826 train rows, 4957 test rows
[1/3] Building vocabulary across 19826 documents...
  Active vocabulary features: 1000
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (12 epochs, 3 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 4957
  Training Time:     123.69s
  Accuracy:        76.96%
  Macro Precision: 0.4522
  Macro Recall:    0.4278
  Macro F1-Score:  0.4300
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               2             1             0             
2              312           555           5             
1              274           3502          22            
0              26            260           1             
============================================================

Successfully saved trained model artifact to: /home/oops/models/archive_hsosd-json-2-model.json

```


///////////////////////////////////////////////////////


note:

Binary works better:
$ cargo run --release -- \
  --mode train \
  --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl \
  --text-col text \
  --label-col label \
  --jsonl \
  --model-type flat \
  --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v3-model.json \
  --epochs 24 \
  --clauses 150 \
  --threshold 80 \
  --specificity 5 \
  --max-features 2500
    Finished `release` profile [optimized] target(s) in 0.05s
     Running `target/release/granmo_windowed_nlp --mode train --train /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --text-col text --label-col label --jsonl --model-type flat --model-path /home/oops/models/Cyber_Bully_Data_binary_class-v3-model.json --epochs 24 --clauses 150 --threshold 80 --specificity 5 --max-features 2500`
Loading training dataset from: /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl
  Total records loaded: 99879
Splitting dataset (99879 total rows) into 80% train / 20% test...
  Split: 79903 train rows, 19976 test rows
[1/3] Building vocabulary across 79903 documents...
  Active vocabulary features: 2500
[2/3] Pre-computing flat BOW vectors (baseline path)...
[3/3] Training flat VanillaTM (24 epochs, 2 classes)...

============================================================
               Classification Evaluation Report             
============================================================
  Evaluated Samples: 19976
  Training Time:     3220.57s
  Accuracy:        91.03%
  Macro Precision: 0.9123
  Macro Recall:    0.9104
  Macro F1-Score:  0.9102
------------------------------------------------------------
Confusion Matrix (Rows: Actual, Columns: Predicted):
               1             0             
1              9410          542           
0              1250          8774          
============================================================

Successfully saved trained model artifact to: /home/oops/models/Cyber_Bully_Data_binary_class-v3-model.json


/////////////
