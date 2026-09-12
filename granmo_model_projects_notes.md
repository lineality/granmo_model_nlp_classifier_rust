[Note: this is a semi-deprecated planning thread that started with very very preliminary convolution model testing and ended up focusing on A. byte-bag type modeling with B. more extensive testing, dataset, and model inspection features.]

Production Rust Coding, Production Data Science. You are professional, detailed (not terse), proactive and helpful; you are not casual, vague, dramatic, or recreationally social.
Production Rust Project:
 (Hand off Docs for Session 5; see code-so-far and 4th hand-off doc below)


Rust project: 
- new test results to analyze: various batch runs... run.
- new dataset cleaner used for latest tests (with best results too)
- task: setting next steps: features & tests (batch v2)
- what features to work on (team diversity)
- what test to modify or add.
- probably focusing on byte-bag for this type of NLP (but other projects might use convolution)
- the preprocessing variation results are interesting

# Goals: Part R&D, Part Production Rust
### R&D Goals
1. Analyze Compute-Optimized Granmo Models for use in Rust Production Data Science including NLP, audio, image, etc.
2. Examine the feature-derivation phase of modeling (e.g. not relying on human pre-made features)
3. 
4. Compare specific preprocessing and model-types e.g. using (existing) batch-comparison:
- flat-BOW-TFIDF
- Byte-Bag
- Byte-Conv
*
- raw
- p0-p5
5. Try out and test new approaches such as:
- feature derivation
- Trade-off-separation (Ensemble?)


Plan: It is easier to pair-down than to re-write, so model-testing code should be future-proofed for production.

### Production Goals
1. A slim language-detection model, e.g. 15-languages, whatever best-balances optimized prediction/inference, and accuracy. (maybe simple multi-class TF-IDF)

1. Highly specialized Rust code only for prediction/inference for a do-one-thing-well built-in-weights binary. (E.g. NLP Sentiment or hate/bullying classification)
2. Production code only for prediction/inference that loads an adjacent file
3. Production Code with both inference and re-train/continuous-train features.



Session 4 Alignment — Test Suite 1.0
possible scope areas:
-  A. Reproducibility & Versioning
Every run produces a run manifest (JSON or plain text, always appended to a log directory):

B. Ablation Matrix Registry
A human-readable json/toml file declaring the rows you will run and their success criteria:
D. more extensive test-error logging (maybe gated learning issue-found logging)
C. Cross-Dataset Agreement Table
E. Fire-Rate & Clause-Role Report (per Session 2)

need two more datasets to establish whether ByteBag is better overall

overall compare approaches across several datasets:
- Byte-Bag(Flat)
- Convolution
- Flat-BOW-TFIDF
- Cluster+TFIDF
- Windowed-BOW-TFIDF

Performance 
A. In this implementation
B. On these data
C. On this type of problem

1. This implementation is likely not balanced or optimized (or properly implemented for experimental scope)
2. These data (ascii-focus) may have non-general properties
3. Language-Hygiene classification is one specific niche; other problem-geometries will be different; regression, probability, time series, embedding-vectors, (unsupervised clustering?), multi-class-classification, unbalanced datasets, FP .vs FN, etc.

That said: Byte-Bag-Flat scores

# Note: parallelization was added.


Byte-Convolution Any-Series Team-Granmo-Automata-Games:
- NLP
- this is for a production-code project
- this particular use case (for this MVP) is bad-language (hate, bullying, etc.) in social-media text.
- this first-MVP is a training-module (focused on establishing feasibility of convolutional Granmo model for NLP), but assuming the models are useful, production-prediction versions (for inference, not training) should be easily adapted from this code. Other implied future scope includes: 
1. a single-prediction feature
2. a training-coach program that tests and compares parameters for best results 
3. gated-learning that checks against benchmarks before retaining learning
4. a continual-learning application that can be updated with new training sets (also gated-learning): this would combine production-prediction with learning (like this application) 
5. a continual-stream inspection application
etc.

- there is a more standard Granmo TM pipeline MVP that is effective, code is available. But that is very MVP and not production-design strict.
- below are various notes including 'Rust Rule' and the Mode & Case-handling framework description, the past-hand-off and the current one. 
- It is entirely possible, it is likely, that there are issues with the existing code, such as bloat or a design that is not future-proofed for where the project is intended to go. Refactoring the existing code should come before
- There are still some recent design additions/corrections made, such as:
22. Parameters: TeamCompositionPalette & Diversity
- Question: Can, should, there be a new ~'model training parameter' (or more than one) for: board_species_team_diversity, to adjust how much variation there is properties of the agents in the game 
(claud says) 
```Should we plan a team-diversity parameter? Plan yes — but as a palette, not a scalar
Your phrasing is clear, and the concept is sound (it is exactly the M-Hetero track: fast/slow "scout/anchor" composition, already validated as theoretically safe for P4-tier properties). Three design points:
2a. It is a construction parameter, not a training parameter. Team composition is fixed at engine construction and must be persisted in the artifact (inference needs per-clause depth N to interpret states; validation needs it for the [1, 2N] band check). So it belongs with ClauseCount/StatesPerAction in the config surface, and heterogeneous artifacts require a format v2 or new kind — a later, clean bump.
2b. Recommend an explicit palette over a scalar diversity knob. A single board_species_team_diversity scalar under-specifies two things: diversity of what (depth? specificity? guard?) and distributed how. Instead:
- Engine-level representation: a TeamCompositionPalette — a small bounded list of species, each species = (clause fraction, StatesPerAction, SpecificityThresholds, guard flag). Homogeneous baseline = single-species palette (the degenerate case, so current behavior is preserved exactly). Deterministic assignment of clauses to species (e.g., contiguous rank blocks within each polarity), so the artifact is fully self-describing and every run reproducible from config alone.

- The scalar you asked about lives one level up, in the future training-coach (future scope item 2): a coach-side palette generator that maps diversity ∈ [0, 1] + a base config + a seed to a concrete palette. That keeps the engine explicit and auditable while giving the coach a single sweepable axis. Best of both.
This also pays off diagnostically: with explicit species, the fire-rate report (scheduled for 2.2c) can histogram per species, which is precisely the instrument for the recorded vote-imbalance risk (deep anchors fire reliably, shallow scouts fire noisily) that decides whether CoTM weights get pulled forward.
2c. What to do now vs. later. Locked decision S2-9 stands: no heterogeneous runs before homogeneous baselines of both engines, or the ablation matrix is confounded. So:
- Now (Drop 3.0): R2 accessors only. No palette type, no CLI flag, no artifact change.
- At M-Hetero: add TeamCompositionPalette + artifact v2 + per-species fire-rate reporting. By then it is storage + accessor-body changes only.
- Record now: the palette-not-scalar decision and the coach-side generator placement, so this doesn't get re-litigated.



- Suggest a manageable set of code at a time, scope creep dilutes focus.
- Planning first and agreeing on scope before coding.
- 'Describing what a change is' is good, but do not mix that description with the overall instructions. spot-changes and individual line changes are fine. e.g. it is wasteful (and error prone) to regurgitate pub fn is_retryable (or enum GranmoModelError ) each time there is a spot-change. Be clear.

- If you need more information about something, ask.



see:
- https://arxiv.org/abs/1804.01508 ,
 "T-Machine: A Game Theoretic Bandit Driven Approach to Optimal Pattern Recognition with Propositional Logic" 2018, by Ole-Christoffer Granmo


- https://arxiv.org/html/2507.14874v2
2025, Gramno et al: The Tsetlin Machine Goes Deep: Logical Learning and Reasoning With Graphs
(There are, amazingly, some useful items from this paper for this scope, but mostly this is not in scope. See below for where this paper helped confirm design decisions.)


- Existing convolution code:
https://github.com/RAprogramm/tsetlin-rs/blob/main/src/convolutional.rs


 - Prior experiment code:
https://github.com/lineality/granmo_model_nlp_classifier_rust

 - Mode & Case Handling in Rust (Specific Rules)
- General Rust Coding Rules

### "Granmo Models" a Descriptive-Terminology Note:
I do understand that in 2018 Gramno magnanimously nick-named his creation in honor of a Russian mathematician (rather than himself). However, in 2026 many of us are living in mortal danger of Russian military belligerence, and it is traumatizing to be expected to "honor" the maniacal forces actively trying to kill us. With or without this context, it is my long held general practice to use clear descriptions instead of jargon terms (not popular, but more clear): these are models made by Granmo, so they are accurately described here as "Granmo Models." Refer to Granmo's models here as "Granmo Models." Use of 'T-Machine' 'TM' or convolutional 'CTM' is nearly-acceptable if unavoidable. Note: Not every automaton is a T-Automaton, not every Machine is a T-Machine. Only use T if it is absolutely unavoidable. A-Machine or the like may be a more accurate description. 



TODO:
1. In at least one place (probably more) there needs to be better ~error/case handling messaging.
- When an invalid path is input there is no message, error, etc. silence (not helpful). (maybe there is no input-path error yet? in theory cli-argument issues and path issues should all have clear error codes)
- Testing is not a 'production loop that needs to run forever', exiting with a code and message is appropriate. if error/case is logged, perhaps print a path to that log on exit.
- Perhaps return (and print to terminal) an exit-code that is either 0 (ok-doky) or the error code


There should also be a glossary (perhaps for the readme if not the .rs module) cleaning explaining terms such as:
- fire-rate
- byte-bag
- 

Note: after adding v1 seed-repeatable parallelism: 
```
**Behavior-changing, expected.** The RNG contract change alters training *trajectories* (not semantics). These three stochastic-outcome tests may need retuning exactly as their own Test-Notes anticipate:

- `learns_byte_level_negation_micro_corpus`
- `bag_learns_negation_micro_corpus`
- `harness_end_to_end_on_negation_corpus` / `harness_bag_engine_end_to_end_on_negation_corpus`
```

Note: This is the current output:
1. The default starter-settings I lowered for quick-test-start:

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


 
vs. sort of similar-ish parameters (not all are comparable)

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

# Batches
(Note: Obviously, more tests on more datasets are needed.)

During improved dataset cleaning-deduplication, fuzzy-duplicate rows (string distance) as well as exact duplicates were removed.

Two areas to look at to gain insights about 'what' and 'how' is happening in 'byte-bag' vs. 'Conv' type models are:
1. Preprocessing: p0-p2 (maybe others should be added or a separate-batch, e.g. p1, p4, p5). Perhaps predictably, byte-bag is 'juiced' by the ability to 'tokenize' standard ascii patterns (classic NLP). Whereas the 'conv' model may prefer less 'word' oriented problem-space geometries.

2. Data-Issues: Comparing before and after deduplication, Byte-Bag was more ~'resiliant to imprecise data' than 'Conv' type, in that Byte-Bag's artificial-boost in performance from duplicate lines appearing as leaks in the test set (and perhaps issues from duplicates in training). Conv's performance (for this one dataset, as much as that can be read into) was roughly 70-80% "accuracy" on dirty data, and 50-60% on cleaned. Byte-Bag was 98.3-99.1% accuracy on dirty data vs. 96.1-97.7% accuracy on cleaned data.

(And, for byte-bag perhaps more always-fires issues using the cleaned data:
dirty-data:


cleaned-data:
never 0/600  
always 40/600 to 50/600 

dirty-data:
never 0/600  
always 23/600 to 25/600
)

Also, the type of preprocessing that did best might also have changed.
Dirty Data: P1
Clean Data: P0

I have not commonly heard that 'deliberately testing with bad, leaky, data' can be useful, but such comparisons may be useful in seeing how a model behaves. (There may also be real-life situations where perfectly-clean data may be infeasible.)

 

# Take 3
- not fully deduplicated

No guard
```
$ cargo run --release --   --mode batch   --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl   --model-out /home/oops/models/batchcyber_conv_v3.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/oops/models/batchcyber_conv_v3.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto`
batch over 79903 train / 19976 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     579.49s
------------------------------------------------------------
  Accuracy (@ V > 0): 70.98%
  Best-F1 Threshold:  V > 10
  Precision:          0.7342
  Recall:             0.8194
  F1-Score:           0.7745
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7021        2964        
Actual Pos (1)    1804        8187        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 16/600  always 71/600  p25 9.2%  median 17.2%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     504.85s
------------------------------------------------------------
  Accuracy (@ V > 0): 98.32%
  Best-F1 Threshold:  V > -2
  Precision:          0.9924
  Recall:             0.9745
  F1-Score:           0.9833
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9910        75          
Actual Pos (1)    255         9736        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 24/600  p25 10.6%  median 26.0%  p75 27.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     562.41s
------------------------------------------------------------
  Accuracy (@ V > 0): 63.07%
  Best-F1 Threshold:  V > -35
  Precision:          0.5571
  Recall:             0.9512
  F1-Score:           0.7027
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    2430        7555        
Actual Pos (1)    488         9503        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 37/600  always 96/600  p25 4.7%  median 14.0%  p75 27.4%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     468.79s
------------------------------------------------------------
  Accuracy (@ V > 0): 99.08%
  Best-F1 Threshold:  V > 1
  Precision:          0.9961
  Recall:             0.9859
  F1-Score:           0.9909
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9946        39          
Actual Pos (1)    141         9850        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 25/600  p25 12.9%  median 25.8%  p75 27.2%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     560.69s
------------------------------------------------------------
  Accuracy (@ V > 0): 67.56%
  Best-F1 Threshold:  V > 19
  Precision:          0.7492
  Recall:             0.7994
  F1-Score:           0.7735
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7311        2674        
Actual Pos (1)    2004        7987        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 14/600  always 82/600  p25 8.5%  median 18.2%  p75 26.9%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     503.12s
------------------------------------------------------------
  Accuracy (@ V > 0): 98.26%
  Best-F1 Threshold:  V > 2
  Precision:          0.9932
  Recall:             0.9720
  F1-Score:           0.9825
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9919        66          
Actual Pos (1)    280         9711        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 27/600  p25 10.6%  median 26.2%  p75 27.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     570.81s
------------------------------------------------------------
  Accuracy (@ V > 0): 61.52%
  Best-F1 Threshold:  V > -23
  Precision:          0.5573
  Recall:             0.9321
  F1-Score:           0.6975
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    2586        7399        
Actual Pos (1)    678         9313        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 35/600  always 88/600  p25 4.0%  median 12.1%  p75 27.2%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     456.46s
------------------------------------------------------------
  Accuracy (@ V > 0): 99.10%
  Best-F1 Threshold:  V > 1
  Precision:          0.9963
  Recall:             0.9862
  F1-Score:           0.9912
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9948        37          
Actual Pos (1)    138         9853        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 23/600  p25 12.9%  median 25.4%  p75 26.9%
============================================================

```

Guard
```
$ cargo run --release -- \
  --mode batch \
  --data /home/ABC/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl \
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
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_v2.jsonl --model-out /home/ABC/models/batchcyber_conv_v3.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --guarded --seed 42 --workers auto`
batch over 79903 train / 19976 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     600.13s
------------------------------------------------------------
  Accuracy (@ V > 0): 79.11%
  Best-F1 Threshold:  V > -1
  Precision:          0.7676
  Recall:             0.8318
  F1-Score:           0.7984
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7469        2516        
Actual Pos (1)    1680        8311        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 11/600  always 82/600  p25 9.4%  median 19.0%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     543.61s
------------------------------------------------------------
  Accuracy (@ V > 0): 98.32%
  Best-F1 Threshold:  V > -2
  Precision:          0.9924
  Recall:             0.9745
  F1-Score:           0.9833
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9910        75          
Actual Pos (1)    255         9736        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 24/600  p25 10.6%  median 26.0%  p75 27.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     622.59s
------------------------------------------------------------
  Accuracy (@ V > 0): 62.49%
  Best-F1 Threshold:  V > 23
  Precision:          0.6723
  Recall:             0.7453
  F1-Score:           0.7069
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    6356        3629        
Actual Pos (1)    2545        7446        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 42/600  always 83/600  p25 4.1%  median 13.5%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     513.21s
------------------------------------------------------------
  Accuracy (@ V > 0): 99.08%
  Best-F1 Threshold:  V > 1
  Precision:          0.9961
  Recall:             0.9859
  F1-Score:           0.9909
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9946        39          
Actual Pos (1)    141         9850        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 25/600  p25 12.9%  median 25.8%  p75 27.2%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     613.13s
------------------------------------------------------------
  Accuracy (@ V > 0): 73.26%
  Best-F1 Threshold:  V > 3
  Precision:          0.7177
  Recall:             0.8057
  F1-Score:           0.7592
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    6819        3166        
Actual Pos (1)    1941        8050        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 18/600  always 78/600  p25 8.6%  median 18.6%  p75 27.1%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     523.54s
------------------------------------------------------------
  Accuracy (@ V > 0): 98.26%
  Best-F1 Threshold:  V > 2
  Precision:          0.9932
  Recall:             0.9720
  F1-Score:           0.9825
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9919        66          
Actual Pos (1)    280         9711        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 27/600  p25 10.6%  median 26.2%  p75 27.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-conv)
  Train/Test Split:  79903/19976 samples
  Training Time:     597.90s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.00%
  Best-F1 Threshold:  V > -44
  Precision:          0.5582
  Recall:             0.9400
  F1-Score:           0.7005
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    2552        7433        
Actual Pos (1)    599         9392        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 39/600  always 93/600  p25 4.2%  median 14.5%  p75 27.5%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79903/19976 samples
  Training Time:     488.69s
------------------------------------------------------------
  Accuracy (@ V > 0): 99.10%
  Best-F1 Threshold:  V > 1
  Precision:          0.9963
  Recall:             0.9862
  F1-Score:           0.9912
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    9948        37          
Actual Pos (1)    138         9853        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19976 test docs: never 0/600  always 23/600  p25 12.9%  median 25.4%  p75 26.9%
============================================================



---

# take 4
- Deduplicator-cleaner has been make and used on this dataset, 
thousands of exact duplicates and hundreds of near-duplicates (found with Levenshtein) were removed.

- de-duplicated cleaned jsonl (sure-load) dataset

$ cargo run --release --   --mode batch   --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v3.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --model-out /home/oops/models/batchcyber_conv_v3.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto`
batch over 79788 train / 19948 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     542.97s
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
  Training Time:     475.93s
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
  Training Time:     547.37s
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
  Training Time:     437.31s
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
  Training Time:     575.90s
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
  Training Time:     486.97s
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
  Training Time:     558.05s
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
  Training Time:     444.90s
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

Guard

$ cargo run --release --   --mode batch   --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v3.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --model-out /home/oops/models/batchcyber_conv_v3.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto`
batch over 79788 train / 19948 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     542.97s
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
  Training Time:     475.93s
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
  Training Time:     547.37s
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
  Training Time:     437.31s
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
  Training Time:     575.90s
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
  Training Time:     486.97s
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
  Training Time:     558.05s
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
  Training Time:     444.90s
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

oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ ^C
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release --   --mode batch   --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v3.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto --guarded
    Finished `release` profile [optimized] target(s) in 0.02s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --model-out /home/oops/models/batchcyber_conv_v3.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto --guarded`
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release --   --mode batch   --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v4.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto --guarded
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --model-out /home/oops/models/batchcyber_conv_v4.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto --guarded`
oops@fedora:~/code/granmo_model_nlp_classifier_rust/para_byte_ganmo$ cargo run --release --   --mode batch   --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl   --model-out /home/oops/models/batchcyber_conv_v4.gmb   --clauses 600   --vote-threshold 160   --stride 1   --patch 5   --specificity 3.0   --states 200   --vocab-size 4000   --ngram-len 5   --epochs 12   --seed 42   --workers auto --guarded
    Finished `release` profile [optimized] target(s) in 0.00s
     Running `target/release/para_byte_ganmo --mode batch --data /home/oops/datasets/NLP/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl --model-out /home/oops/models/batchcyber_conv_v4.gmb --clauses 600 --vote-threshold 160 --stride 1 --patch 5 --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 --epochs 12 --seed 42 --workers auto --guarded`
batch over 79788 train / 19948 test documents, seed 42

============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-conv)
  Train/Test Split:  79788/19948 samples
  Training Time:     563.51s
------------------------------------------------------------
  Accuracy (@ V > 0): 65.17%
  Best-F1 Threshold:  V > -43
  Precision:          0.7566
  Recall:             0.8365
  F1-Score:           0.7945
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7325        2677        
Actual Pos (1)    1626        8320        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 9/600  always 94/600  p25 10.1%  median 19.9%  p75 26.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        raw          (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     490.18s
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
  Training Time:     564.62s
------------------------------------------------------------
  Accuracy (@ V > 0): 56.51%
  Best-F1 Threshold:  V > -57
  Precision:          0.6656
  Recall:             0.7562
  F1-Score:           0.7080
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    6223        3779        
Actual Pos (1)    2425        7521        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 30/600  always 105/600  p25 4.6%  median 18.6%  p75 26.8%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p0           (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     456.24s
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
  Training Time:     541.31s
------------------------------------------------------------
  Accuracy (@ V > 0): 57.76%
  Best-F1 Threshold:  V > -56
  Precision:          0.7289
  Recall:             0.8033
  F1-Score:           0.7643
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    7030        2972        
Actual Pos (1)    1956        7990        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 12/600  always 107/600  p25 9.6%  median 20.9%  p75 27.0%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p1           (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     465.83s
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
  Training Time:     542.48s
------------------------------------------------------------
  Accuracy (@ V > 0): 54.48%
  Best-F1 Threshold:  V > -65
  Precision:          0.6334
  Recall:             0.7765
  F1-Score:           0.6977
------------------------------------------------------------
Confusion Matrix (at optimal threshold):
                  Pred Neg (0)Pred Pos (1)
Actual Neg (0)    5532        4470        
Actual Pos (1)    2223        7723        
------------------------------------------------------------
Clause Dynamics:
  fire-rate over 19948 test docs: never 43/600  always 113/600  p25 4.1%  median 18.5%  p75 27.0%
============================================================


============================================================
               Classification Evaluation Report             
============================================================
  Run Preset:        p2           (Engine: byte-bag)
  Train/Test Split:  79788/19948 samples
  Training Time:     436.67s
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


commentary by Claude:
```
Reading the Batch Results
First, the headline findings (these are significant)

Finding 1 — M-ByteBag gate answered decisively: sub-word granularity alone is a massive win. The byte-5-gram bag at F1 0.965–0.983 crushes the Phase-1 word-BOW comparator (~0.82) on the same dataset. That answers the original Milestone-1 gate question ("does sub-word granularity alone move accuracy?") — yes, dramatically. This is a publishable-quality internal finding regardless of what happens with convolution.

Finding 2 — S2-7 diagnostic, current position: Conv(no-guard) < Bag, by a wide margin (F1 ~0.80–0.84 vs ~0.965–0.983). Per the locked diagnostic table, this pattern has two possible readings, and we cannot yet distinguish them because the guard arm hasn't run:

defect is real and the guard repairs it → run Conv(guard) to test, or
convolution is not earning its complexity on this task.

Finding 3 — the conv fire-rate report corroborates the dead-clause hypothesis. Conv shows 12–28/200 never-firing clauses across presets (bag: 0/200 in every run), plus best-F1 thresholds displaced well off zero (−15 for raw) on a balanced dataset (your value_counts confirms 50/50) — that is vote imbalance, exactly the signature the mutual-exclusion defect predicts (dead/starved clauses concentrated unevenly across polarities).

Before concluding "conv loses" — one confound must be controlled: capacity asymmetry

This comparison is not yet capacity-fair, and it's worth being explicit:

A conv clause can express essentially one K-byte positional pattern (a slot with a positive include allows one byte). 200 clauses = at most ~100 positive-polarity patterns total.
A bag clause draws literals from a 2000-shingle vocabulary and can require/forbid combinations. Its effective feature space is 10–20× richer at these settings.

So the bag isn't just "non-positional vs positional" here — it's also "2000 features vs ~100 pattern slots." The §8 single-variable discipline requires we separate those. Secondary confounds: stride 2 (patterns at odd offsets need duplicate clauses — the recorded S2 cost) and possibly epoch count for the noisier conv feedback (single-sampled-window attribution).

Proposed next runs (in priority order, your time budget permitting)

Conv runs take ~8 min each at 200 clauses; clause count scales roughly linearly.

Conv + --guarded, p0, otherwise identical (~8 min) — directly resolves the S2-7 ambiguity. This is the single most informative run available.
Conv, 1000 clauses, p0, no guard (~40 min) — the capacity-fairness control. If conv at 1000 clauses still sits far below bag-at-200-clauses/2000-vocab, the "not earning complexity" reading strengthens materially.
Conv, stride 1, p0 (~16 min, double scan cost) — quantifies the recorded S=2 parity cost.
Optionally the combination (guard + 1000 clauses + S1) as the conv best-case.

If 1–3 all fail to close the gap substantially, the honest §8 conclusion is the one the plan explicitly anticipated: "If ByteConvTM only matches [or loses to] the flat byte-n-gram baseline, the convolution machinery is not earning its complexity — that is itself a valid finding" — and the crate's production value pivots to the ByteBag engine (which is genuinely strong) with conv retained for the stream-evaluator use cases where positional/streaming structure matters.

Two smaller observations to record:

Bag preset ordering: p2 (leet-fold) ≥ p0 > raw > p1 — the advance predictions held (folds help, no-lowercase hurts at V>0 though its best-F1 recovers via threshold). Worth noting p1's accuracy@V>0 (0.81) vs best-F1 (0.966) gap: the default V>0 boundary is misleading when votes are offset; best-F1 rows are the comparable numbers.
Harness bug to log: batch mode silently accepted and ignored --model-out. Under the fail-fast philosophy that should either warn or be rejected in batch mode. Small fix, can go in the next drop.

Shall I hold here while you run #1 (and #2/#3 if budget allows), or would you like me to proceed to your item 2 (use-case divergence: flat BOW-TFIDF vs byte-stream-convolution) while those run?
```

Side Task:
- If abstract or speculative, I want to try to list out where use-cases might exist and diverge between 
A. bow-tfidf flat traditional Ganmo-Model
B. byte-stream-series-convolutional Ganmo-Model

E.g. a the byte-stream-series might a flexible for more use-cases:
- IoT data time series
- financial data time series
- audio-time series
- more diverse character-sets/languages, mixed-languages, etc.
- deployment: continual stream reporting
- deployment: variable size input flexibility


E.g. another interest area is No-GPS direction-change detection from continual input stream, very low-power computation:
- could a byte-stream convolution model try to operate on (some derivative of video or sensor input?)


Not to distract from scope, but here are the overall notes for the larger set of Granmo model test modules, including clustering, 'flat'BOW-tfidf, windowed-bow, and now byte-convolusion-stream-diverse-team-games
```


[Hand off Doc 1 written (mostly) by Claude]
```
## 1. Project Goal and Scope

Build and evaluate a **byte-level convolutional Tsetlin Machine** for **binary classification of short social-media text (hate/bullying detection)**, producing a probability output, with these hard constraints:

- **Production profile:** integer-only hot paths (no floats in training inner loop or inference), small deployable artifact (target: tens of KB), single-pass streaming-capable inference.
- **Explainability:** fired clauses decode to human-readable byte patterns with exact source-span offsets.
- **Scientific discipline:** every architectural change is benchmarked against retained baselines on identical splits with fixed seeds.

**Out of scope for this phase:** multi-class, generative anything, GPU, Platt calibration, bit-parallel/SIMD optimization, UTF-8 boundary alignment, multi-granular patch sizes.

---

## 2. Status: Existing Code Inventory and Assessment

Phase 1 code (`main.rs`, single crate, deps: serde/serde_json/csv/rayon) provides:

| Component | Status | Phase 2 role |
|---|---|---|
| `PipelineError` | Good | Reuse as-is; add variants only if needed |
| `FastRng` (xorshift64) | Good, deterministic | Reuse; **add integer-threshold draw methods** (see §6) — `gen_f64` stays only for legacy models |
| `Dataset` (CSV/JSONL, label alignment, seeded split) | Good | Reuse as-is |
| `Vocabulary` (word-level) | Good | Reuse for word baselines; **add byte-n-gram view** for M-ByteBag |
| `VanillaTM` (flat BOW) | Working, best current accuracy | **Ablation control — do not modify** |
| `WindowedTM` (word-level conv) | Working, underperforms flat | **Ablation control — do not modify** |
| `EvaluationReport` | Good | Reuse; add per-class + margin-sweep output later if needed |
| `PipelineModel` / `ClassifierModel` enum | Good pattern | **Add `Byte(ByteConvTM)` variant**; JSON artifact fine for experiments |
| Explainability (`FiredRule`, positional traces) | Good pattern | Extend for byte spans |
| CLI (fail-fast parsing, train/predict modes, demo) | Good | Extend with preprocessing + byte-model flags |
| Tests (invariant validation, learning tests, round-trip) | Good pattern | Replicate style for new engine |

**Key finding from code inspection (this resolves former diagnostic milestone M-Diag):** `WindowedTM` **is** correctly weight-shared across document positions — one clause with `(relative_slot, word_id)` literals is slid over all window starts, with single-sampled-window Type Ia/II feedback. The convolution mechanics are already right. Its structural weakness is the **literal space per clause: `2 × W × V` (e.g., 24,000 literals at V=4000, W=3)** — each window slot carries a full one-hot vocabulary, so exactly one of ~4000 positive literals is true per slot, Type Ia/II feedback costs O(W×V) per event, and clause convergence is starved. **The byte model keeps the (correct) convolution mechanics and shrinks the per-slot alphabet from V≈4000 to 256**, giving `2 × K × 256 = 2,560` literals at K=5 — roughly 10× smaller, with no OOV concept at all. This is the central hypothesis of Phase 2.

---

## 3. Established Technical Facts (verified against primary sources)

These were double-checked against Granmo et al., *The Convolutional Tsetlin Machine* (arXiv:1905.09688) and Granmo et al., *The Tsetlin Machine Goes Deep: Logical Learning and Reasoning With Graphs* (arXiv:2507.14874). Treat as settled:

1. **OR-pooling is canonical.** A clause's document-level output is 1 iff it matches at least one window. GraphTM independently confirms (graph output = OR over nodes). OR-pooling is also length-invariant — important because document length varies, unlike images. The Phase 1 `CountFire` (capped sum) pooling remains available as an ablation only.
2. **Feedback when clause fired (Type Ia reinforce / Type II suppress): sample ONE fired window uniformly** and apply literal updates against it. Phase 1 already implements this correctly.
3. **Feedback when clause did not fire (Type I, clause=0 branch) is input-independent**: uniform push toward exclusion with probability 1/s. **No window selection is needed or meaningful.** Phase 1's `apply_type_ib_feedback` already does this correctly.
4. **Convolutional TM = GraphTM with disconnected nodes, depth 1** (verified empirically in the GraphTM paper). Our byte-conv model is the base case of that ladder; depth-2 message passing is a defined escalation path (backlog, §12).
5. **Coalesced TM weighted clauses** (integer per-class clause weights, ±1 updates) achieve comparable accuracy with fewer clauses and remain integer-only. Cheap orthogonal upgrade — ablation backlog.
6. **Length neutrality of the chosen design:** with OR-pooling, each clause contributes at most 1 vote and at most one feedback event per document regardless of length; length affects only scan cost and spurious-match odds. A configurable `max_scan_bytes` cap must exist.

---

## 4. Model Specification of Record: `ByteConvTM`

**Input:** preprocessed byte sequence (§5). Windows of **K bytes (default 5)**, **stride S (default 2; S=1 configurable)**, slid over the document. Documents shorter than K are right-padded with **0x00** (decided; never occurs in normal text). One-hot-256 encoding is *implicit*: the byte value is the literal index; no expanded vectors are materialized.

**Clause structure:**
- One clause = K positions × 512 automata: positive literal `(k, b)` = "byte at window offset k equals b"; negated literal `(k, b)` = "byte at offset k is not b". Literal layout mirrors Phase 1: positive at `k*256 + b`, negated at `K*256 + k*256 + b`. States are i16, include iff state > `states_per_action`.
- One shared clause slid over all window positions (as in Phase 1).

**Derived evaluation structure (design commitment, not an optimization afterthought):** per clause, per offset k, a 256-bit **allowed-bytes mask** (`[u64; 4]`): bit b set iff byte b passes both the positive and negated constraints at offset k (positive-include of b' ≠ b ⇒ b disallowed; negated-include of b ⇒ b disallowed). Clause matches a window iff all K observed bytes are allowed — **K bit-tests per clause per window, zero floats**. Masks are recomputed only when an automaton crosses the include/exclude boundary (route all state changes through increment/decrement methods, exactly as Phase 1 does with `included_positive_counts`; re-validate on model load, same pattern as `validate_internal_consistency`).

**Bank and vote (binary task — simpler than Phase 1's per-class banks):** single pool of `n_clauses` (default 200), even indices positive polarity (class 1), odd negative (class 0). `V = Σ fired(+) − Σ fired(−)`; label = `V > decision_threshold` (default 0, sweepable). Training uses standard clamp to ±T (default 50) and the `(T∓V)/2T` update probabilities — implemented with integer comparisons (§6).

**Forward pass modes:** `predict` may early-exit per clause on first match; `train` needs one uniformly sampled fired window — implement via reservoir sampling in a single pass (count matches, keep current sample with probability 1/count) so no `Vec<usize>` of fired positions is allocated per clause per step (improvement over Phase 1's `fired: Vec<usize>`).

**Feedback:** identical logic to Phase 1's `WindowedTM` (correct per §3), but per-event cost drops from O(W×V) to O(K×256) = 2,560 literal visits — cheap.

**Footprint (K=5, 200 clauses):** training state ≈ 1.0 MB (i16); masks ≈ 32 KB. **Inference-only artifact = masks + polarities (+ optional weights + probability LUT + preprocessing config header) ≈ tens of KB — an explicit deliverable.**

**Config surface (extends `BenchmarkConfig`):** `patch_size` (K), `stride`, `n_clauses`, `threshold` (T), `specificity` (s), `states_per_action`, `pooling` (OR default; capped-count as ablation), `max_scan_bytes`, `seed`, `epochs`, plus the preprocessing profile (§5). All resolved config values are persisted in the model artifact and echoed at train time.

---

## 5. Preprocessing Specification

**Principles:** (1) every stage is a byte-stream adapter — bytes in, bytes out, O(1)/bounded state, so the pipeline works on streamed input; (2) the enabled-stage list is configuration selected at train time and **persisted inside the model artifact** — inference must replay it exactly (non-negotiable rule); (3) non-ASCII bytes pass through untouched by default (emoji/other scripts are opaque but consistent byte patterns).

**Stage catalogue, canonical order:**

| # | Stage | Transform | State | Default |
|---|---|---|---|---|
| 1 | Whitespace fold | `\n \r \t` → space | none | ON |
| 2 | Space dedupe | collapse space runs | 1 byte | ON |
| 3 | Leading trim | drop leading spaces | 1 bit | ON |
| 4 | ASCII lowercase | bytes 65–90 → +32 | none | ON (A/B tested) |
| 5 | Leet fold | `@→a $→s 0→o 1→l 3→e !→i` (configurable map) | none | OFF, top-priority experiment |
| 6 | Digit strip | remove 0x30–0x39 | none | OFF, experiment |
| 7 | Symbol strip | remove ASCII punctuation (space excluded) | none | OFF, experiment |
| 8 | Space strip | remove all spaces | none | OFF, expected harmful (word boundaries are signal: "\_ass\_" vs "class") |
| 9 | Porter stemmer | word-level suffix strip | bounded word buffer | OFF, late ablation; dependency decision open (§11) |

**Predictions recorded in advance:** folds (lowercase, leet) merge obfuscated and plain spellings — expected wins; strips delete potential signal (digits are leetspeak carriers) — expected neutral-to-harmful; stemming partially redundant with byte windows and unstable on typo-heavy text.

**Experiment presets (each one training run):** `raw` (no stages — streamed-production case, always reported); **P0** = stages 1–4 (baseline); **P1** = P0 minus lowercase (capitalization A/B — ALL-CAPS is genuine signal in bullying detection); **P2** = P0 + leet-fold; **P3** = P2 + symbol strip; **P4** = P0 + digit strip; **P5** = P0 + space strip; **P6** = P0 + stemmer. Priority order: P0, P1, P2 first.

**Deferred idea (recorded, not MVP):** lowercase + inject sentinel byte 0x01 before originally-ALL-CAPS words — normalizes patterns while keeping "shouting" learnable.

---

## 6. Compute Policy: Integer-Only Hot Paths

Hard rule for `ByteConvTM` (existing `VanillaTM`/`WindowedTM` are exempt as frozen baselines):

- Clause evaluation: bit-tests on `[u64;4]` masks only.
- Feedback selection `(T ∓ V)/2T`: draw integer `r ∈ [0, 2T)`, compare `r < T − V` (or `T + V`). Exact, no division at runtime.
- Specificity `1/s`, `(s−1)/s`: precompute `u16` thresholds (≈ `65536/s`) at config time; per-decision compare one random `u16`. `s` remains a float in config files only. **Add to `FastRng`:** `next_u16()`-style draw and a `gen_range` that avoids modulo bias if desired (or document acceptance of modulo bias — decide under coding rules).
- Probability output: `V ∈ [−n_clauses/2, +n_clauses/2]` → precomputed lookup table (sigmoid with τ = T/2, fixed-point u16), filled once at model-save time. Runtime probability = one array index.
- Floats permitted only in offline evaluation/reporting (`EvaluationReport` etc.).

---

## 7. Outputs Beyond the Label

1. **Probability:** LUT sigmoid over V (§6). Calibration refinement (Platt) deferred; would only change how the LUT is filled.
2. **Embedding (free byproduct — deliverable):** the fired-clause bitset (~200 bits, already computed for voting) is a learned, interpretable binary document embedding; expose it in `PredictionOutput` (e.g., as a compact hex string or bit vector) for similarity/clustering/near-duplicate use via Hamming distance.
3. **Explainability:** per fired clause — decoded pattern ("offset 0 must be `i`, offset 1 must be `d`, offset 4 must not be space") + matched byte-span offsets in the source text. Same `FiredRule` structure, byte-span variant. Build this *during* engine development — it is the primary debugging instrument.

---

## 8. Evaluation Plan and Success Criteria

**Dataset (decided):** short social-media hate/bullying text, binary label. Expect class imbalance → report precision/recall/macro-F1 and sweep the integer decision threshold (free: boundary is `V > c`). Reuse `EvaluationReport`.

**Robustness suites (build once, freeze):**
1. **Obfuscation suite:** held-out positives rewritten with leet substitutions, inserted symbols, casing tricks — measures the byte model's core claimed advantage.
2. **Word-boundary/negation suite:** minimal pairs where local order or boundaries flip the label (Phase 1's negation demo is the seed for this).

**Comparison matrix (identical splits/seeds):** flat word-BOW (`VanillaTM`) · word-window (`WindowedTM`) · **flat bag-of-byte-n-grams** (tokenizer swap on `VanillaTM` — isolates "does sub-word granularity alone help" from "does convolution add anything") · **`ByteConvTM`**, across preprocessing presets.

**Success criteria (fixed now):** (a) no macro-F1 regression vs. flat word-BOW; (b) clear win on the obfuscation suite; (c) inference artifact meets §4/§6 production profile. Win on (b) with parity on (a) = success. If `ByteConvTM` only matches the flat byte-n-gram baseline everywhere, the convolution machinery is not earning its complexity — that is itself a valid finding.

---

## 9. Milestones

1. **M-ByteBag** — Byte-n-gram tokenizer view on `Vocabulary` (fixed-length shingles over preprocessed bytes) feeding the unchanged `VanillaTM`. Fastest-training vehicle; run P0/P1/P2 preprocessing experiments here first. *Gate: does sub-word granularity alone move accuracy / obfuscation robustness?*
2. **M-Preprocess** — Byte-stream adapter pipeline (stages 1–8; stemmer stubbed pending §11 decision), preset selection via CLI, config persisted in artifact. Prerequisite for everything; shared by all models.
3. **M-Conv-Core** — `ByteConvTM` per §4 + §6: masks, reservoir-sampled feedback, integer RNG paths, `ClassifierModel::Byte` variant, invariant re-validation on load, unit tests replicating Phase 1's test style (order-sensitivity, PAD semantics, invariant-after-training, save/load round-trip, learning test on a byte-level negation micro-corpus).
4. **M-Explain** — byte-span `FiredRule` decoding + embedding bitset in `PredictionOutput`. Built alongside M-Conv-Core, not after.
5. **M-Prob** — probability LUT + threshold sweep reporting.
6. **M-Ablate** — full comparison matrix (§8) incl. pooling ablation (OR vs. capped-count) and stride 1 vs. 2.
7. **M-Prod-Pass** — compact inference artifact (masks + polarities + LUT + preprocessing header; drop TA states), streaming inference path verification, size/throughput measurement.

---

## 10. Locked Decisions Log

| # | Decision | Value |
|---|---|---|
| 1 | Task | Binary hate/bullying classification, short social-media text, probability output |
| 2 | Pooling default | OR (any-fire); capped-count kept as ablation only |
| 3 | Feedback | Single-sampled fired window for Ia/II; input-independent 1/s decay for clause=0 (Phase 1 semantics preserved) |
| 4 | PAD | 0x00, right-pad when doc < K |
| 5 | Alphabet | Implicit one-hot 256; non-ASCII passes through untouched |
| 6 | Lowercase | ON by default (ASCII-only fold); P1 A/B tests OFF |
| 7 | Hot-path arithmetic | Integer-only for the new engine (§6) |
| 8 | Baselines | `VanillaTM` and `WindowedTM` frozen as ablation controls |
| 9 | Artifact | Must embed its preprocessing config; training echoes resolved config |
| 10 | Max scan length | Configurable `max_scan_bytes` cap exists |

## 11. Open Decisions (answer before/at M-Conv-Core)

1. **K default:** proposed 5 — confirm or change.
2. **Stride default for recorded benchmarks:** proposed 2 (speed), with S=1 as one ablation run — confirm.
3. **Stemmer dependency policy:** `rust-stemmers` crate vs. hand-rolled vs. drop stage 9 entirely.
4. **Config UX:** CLI flags only, or interactive Q&A mode as well.
5. **Weighted clauses (CoTM-style i16 per-clause weight, ±1 updates):** include as M-Ablate arm now, or defer to backlog.
6. **Crate layout:** new module(s) in the existing crate (consistent with current single-file style?) vs. splitting into modules — governed by the coding rules to be supplied.

## 12. Backlog / Future Research Tracks (not Phase 2)

- **GraphTM-lite:** chain-graph, depth-2 message passing over byte windows. Trigger: MVP error analysis shows failures needing context beyond K bytes (e.g., long-range negation). Validated framing: our conv model is GraphTM's depth-1 disconnected special case.
- **Weighted clauses (CoTM)** if not taken in §11.5.
- **Caps-sentinel preprocessing** (0x01 before ALL-CAPS words).
- **TM-autoencoder embeddings** (self-supervised, label-free) — separate research project.
- Bit-parallel/SIMD evaluation; UTF-8-boundary-aligned windows for cleaner non-ASCII explanations; Platt calibration; multi-class.

## 13. Notes for the New Session

- Carry into the new session: **this document + the Phase 1 source + Cargo.toml + the coding rules** (not yet supplied — apply them before writing any code).
- Existing conventions observed in the code and to be preserved unless the rules say otherwise: `#![forbid(unsafe_code)]`, no-panic policy (Result everywhere, no assert in production paths), absolute-path policy for I/O, deterministic seeded runs, invariant caches validated on load, fail-fast CLI parsing, doc-comments explaining *why* not just *what*.
- First concrete actions: resolve §11 items 1–4, then M-Preprocess and M-ByteBag in parallel (both small), then M-Conv-Core.
```



# Other Notes:



Design questions in a context of the 2018 Ganmo paper: 
1. What are the Automata properties needed for a Ganmo-Game (e.g. as satisfied by a Tsetlin Automata, if only for some games)?
2. Could needed-properties vary by Game-Outcome-Task (type)?
3. Could a simpler or different Automata play a given (type of) Ganmo-Game?
4. For example, for a byte-convolution-game, could a more optimal Automata be defined?
5. Diverse Team Composition: Could ~teams of automata with diverse properties perform well for some game-spaces?


1. Versions:
- First MPV: We of course need a train-test-evaluate version.
- Eventually there should be a production-prediction version, e.g. with self-contained pre-compiled 'weights.'
- There may be a 'uses adjacent model in file' production version also oriented towards prediction (but less extremely optimized).
 - There may also be a 'keep training on the fly' production version, etc.

 2. feature goal (if next-project stretch goal): A continual stream evaluator, e.g.
byte-stream convolution modeling,
with some kind of paired-offset-window chunks to average-over start-stop points, e.g.
there are two 128 byte evaluators that return results after 128 byte, which might be in the middle of a word, so, in theory, comparing the output of these two offset by 64byte return schedules, it may be possible to do a continual stream reporting functionality with very light overhead.
 every 128 bytes
- bitwise optimized
- balanced between pre-processing-minimal and performance-best
- results likely sent to some Database or other endpoint for logging/alerting.

The main 'edge' case might be if a half-word somehow creates a false-positive... but that might be highly unlikely for the convolve-process.

 There would likely be double-signals caught by both, which... might be averaged out. But a simple detector might not care: Zero vs. not-zero: two positive signals is still not zero.

Mode and Case Handling Outline:


# Mode & Case Handling Bullet Points Summary
## Framework: Three Modes & Rules

### Three Modes of Operation:
1. Production-Release Mode:
 - Never panics, halts, or leaks data.
- Uses 2-byte error codes (no heap, no strings, no PII).
- Smoothly handles all cases via "Let It Fail & Recover" (with optional logging).
- Uses a Three-Level Recovery Hierarchy

 2. Debug Mode:
- Similar to production but logs verbose diagnostics (heap allowed).
- Uses `eprintln!` for errors and `debug_assert!` for internal invariants (gated to avoid running in test/production).
- Optional debug printing/logging for inspection.

3. Test Mode:
- Uses `assert!` in test functions.
- Deliberately crashes/panics to generate stack traces for debugging.



### Rules:

1. All functions return `Result<T, YourProjectError>`. The error payload is always the 2-byte `YourProjectError` enum. All operations are expected to eventually fail in some way; all such failures must be smoothly handled.

2. Production-Release Mode (Result-Error case-handling):
- never panics
- no heap in error-code-return from functions
- no print or log of that code by the function before the code is returned

3. Separate Debug-Mode Assertions from Cargo-Tests:
- `debug_assert!` must be gated with `#[cfg(all(debug_assertions, not(test)))]` to exclude from test mode and production mode builds.

4. Use Gated Verbose Debug-Mode Diagnostics:
- Use `eprintln!` (gated with `#[cfg(debug_assertions)]`) for debug-only output. Heap is allowed/needed here.

5. Test-Mode Isolation:
- Test code must be gated with `#[cfg(test)]`.
 - Avoid using test-mode tests inside functions for stable code.

6. Use a Fieldless Enum error-code system.

7. Use Enforced-Custom-Types for Value-Integrity.

8. Use a Three-Level Recovery Hierarchy for recovery and state.


### Enforced-Custom-Types & Value-Integrity:
- Use `struct`/`enum`/`impl` to enforce value boundaries for inputs and intermediate values. This ensures invalid states (including bit-flips, corruption) are caught and handled as errors, not passed silently.
- required: add a validity-recheck into the .get() method
- Optionally use a .validity_recheck() method
- While a custom type in Rust can enforce and validate that a value is within the definition boundaries, this check only happens once when the constructor is run ( .new() ), assuming that the constructor was correctly written to carry out that check correctly. If memory-corruption happens after that initial check there are no automatic checks that will catch that the value is now invalid (e.g. as the value is returned from and accepted by another function). Additional .validity_recheck() methods can be made and used to manually check at specific points.
- Using custom types helps to manage compatibility within the design (e.g. narrowing scope of function inputs)
- vanilla custom types (without additional validity-checks) guard against design errors (coding-mistakes) and 'expected' errors.
- additional validity-rechecks can add guards against some electrical, hardware, and adversarial based errors, or 'unexpected errors.'
- Getting and mutation/updating use methods that must be manually designed to check & enforce boundary checks and validity.
#### Public & Private
- note: the individual rules for structs, enums, impls, and their combinations, vary.
- A struct CAN be safely pub (so other modules can use the type name in signatures).
- The struct's inner fields should NOT be pub (so other modules cannot bypass checks).
- Example of safety for combinations: if an enum variant carries data then do not put raw unbounded primitives inside that; rather, put a bounded struct inside.

### Three-Level Recovery Hierarchy ("Let It Fail & Recover")
Production-release failures should fit one of three bounded recovery tiers:
- Recovery Tier 1: Micro-Retry (Local) — If err.is_retryable() is true, a caller repeats the bounded operation (with backoff/sleep).
- Recovery Tier 2: Step Fallback / Safe Degradation (Subsystem) — This is at a level above which any retry-errors would be thrown. If non-retryable, or retry attempts are exhausted, the subsystem safely aborts the current command, and handles what to do next. This is highly case dependent, and might include trying another function, reverting to a previous state, exiting silently, etc. Some internal state may need to be reset. Ultimately move on with or without logging the case or "error."
- Recovery Tier 3: Macro Re-initialization — An outer loop reinitializes state and continues execution without halting. The largest case for Tier 3 reboots the entire program.

#### Recovery & State-Recovery:
Recovery tiers represent functional layers of 'state' in terms of 'recovery levels' to plan for, e.g. what to do when some state-values may not exist. Tier 1 covers the state in question. Tiers 2 and 3 range from small to large 'reboot/retry' scales up to the whole program. Full system restart will likely often have at least some state that it can be restarted with (resulting in no noticeable interruption), sometimes implying that state should be managed outside of retry-loops, to retain intact values.


### Error Handling System
- Single Fieldless Enum:
 A global `YourProjectError` enum (e.g., `#[repr(u16)]`) defines all error codes.
 Properties:
 - No heap (2-byte `Copy` values).
 - No `String` or `io::Error` payloads (prevents PII leaks).
 - Exhaustive `match` for retryability (`is_retryable()` method).
 - Append-only codes: Never reuse or renumber.

- Error Code Table Rules:
 1. Codes are unique and permanent (e.g., `Fs32tFromStrInputTooLong = 101`).
 2. Reserve blocks per module/feature (e.g., 100–199 for `FixedSize32Timestamp`).
 3. Variant names: `AcronymFunctionCondition` (e.g., `IoPermissionDenied`).
 4. Document codes in the enum’s doc comment.

- Display for Debug Only:
 `Display` impl (for human-readable text) is compiled only for debug/test builds.

#### Error Sites & Propagation:
- Unique codes permit use of '?' because the unique code of the root-cause error is preserved (not obscured) by propagating the error.
- If an error site is of note, that site needs to throw a specific error code.
- This acts as a proxy for manually checking for most internal-invariant type issues.


### Two Detection Patterns
1. If-Detection-Pattern:
 For direct condition checks (e.g., `if len > 32`).
 - Debug: `debug_assert!` (internal invariants only) + `eprintln!`.
 - Production: Return terse error code.

2. Match-A-Function-Call-Pattern:
 For handling `Result` from fallible calls (e.g., `std::str::from_utf8`).
 - Debug: `debug_assert!(false, ...)` (for internal invariants) + `eprintln!`.
 - Production: Drop callee’s error (to avoid heap/PII), return project error code.


## Key Principles
- No Heap in Production: Ban `String`, `format!`, `Box<dyn Error>`, etc.
- No Panics in Production: Use `checked_add`, `.get()`, etc., to avoid silent wraps/panics.
- Defensive Programming:
 - Input Validation: Expected issues (e.g., bad user input) → return error code + debug `eprintln!`.
 - Internal Invariant: "Should-not-happen" checks (e.g., bit-flips) → `debug_assert!` + production catch.
- Retry Logic: Defined per error code (not ranges) in `is_retryable()`.


### Example Code
#### Error Enum
```rust
#[repr(u16)]
pub enum YourProjectError {
 IoNotFound = 50,
 Fs32tFromStrInputTooLong = 101,
 RetryMaxAttemptsZero = 200,
}
```

#### If-Detection-Pattern
```rust
if !condition {
 #[cfg(debug_assertions)]
 eprintln!("ACRO-101: detail: {}", value);
 return Err(YourProjectError::AcroFnCondition);
}
```

#### Match-A-Function-Call-Pattern
```rust
match fallible_call(input) {
 Ok(v) => Ok(v),
 Err(_detail) => {
 #[cfg(debug_assertions)]
 eprintln!("ACRO-101: {}", _detail);
 Err(YourProjectError::AcroFnCondition)
 }
}
```

### Banned in Production
- `unwrap`/`expect`/`panic!`
- Heap allocations (`String`, `format!`, `Box<dyn Error>`)
- Error messages with PII (file/dir paths, user data, etc.)
- Unchecked Arithmetic: Arithmetic operators that can panic in production (use `checked_add`, etc.)


#### Suggestions (TODO: under construction; move to rules, for manageable use of codes?)
- Allow effective "blocks" of codes for functions, to allow coherent numbering and easy incrementing in case two developers collide. e.g. if the last two digits are internal to a function, u16 would allow for 654 functions to each have 99 internal errors (or depending on average function size, ten per function may be enough on average).
- alt phrasing: allow 10 or 100 error-codes per function to manage allocations and changes to codes


#### rust_lang_rules
(Production-Rust rules of thumb)

#### Also see [Mode & Case-Handling](https://github.com/lineality/modes_and_case_handling/blob/main/mode_case_handling_framework_summary.md)

# 🦀 Rust Guidelines 🦀:
- Always best practice
- Never ~unsafe code
- Always extensive doc strings: what the code is doing with project-context
- Always clear comments
- Include cargo tests (where possible)
- Functions should return result (in real life everything will fail at some point)
- Never remove (still-current) documentation
- Always clear, meaningful, globally unique names (e.g. variables, functions, etc.): Do not give two things one name, do not give one thing two names.
- Always absolute file paths
- Always error handling
- '?' should be avoided where it would obscure an error
- Never use unwrap (in production-release builds; use in cargo-tests is obviously valid)
- Always handle modes (test, debug, production-release) and cases/errors appropriately for that project
- Always follow boy-scout values
- Single-Flat: Default to a single-flat-file module x.rs. The crate src/ contains two files: main.rs and x.rs. main.rs uses "mod x; use x::FUNCTIONNAME;" to run the module. The single-flat-file module x.rs (where x = the name of the module) is portable to be added to other crates and projects).
- Vanilla-Rust: No third party crates.

# 🦀 10 Rust Rules 🦀:
1. Avoid Risky Methodologies: 
- no recursion  
- no goto 
- no fancy pointer use 
- no preprocessor branching
('unsafe' code blocks in Rust may be unavoidable)

2. Loops: either firmly bounded or unbounded w/ recovery

3. Attempt to Pre-allocate all memory (stack, not heap)
- case by case, avoid heap in production-release-mode where sound

4. Clear Function Scope and Data Ownership

5. Mode & Case Handling, & Defensive-Programming: 
- Modes: test-mode, debug-mode, production-release-mode; 
- continual state-recovery (without panic/halt) in production-release-mode
- See Example/Default Framework: [github.com/lineality/modes_and_case_handling](https://github.com/lineality/modes_and_case_handling)

6. Manage ownership and borrowing

7. Manage return values

8. Manage conditional compilation

9. Communicate: 
- Use doc strings; use comments. 
- Document use-cases, edge-cases, policies, intent: features vs. bugs, etc.
- Rather than let _ =, allow that result to be shown in debugging
- log errors (MVP: append log file in executable-parent dir)

10. Use state-less operations when possible

#### References & Links:
- https://en.wikipedia.org/wiki/The_Power_of_10:_Rules_for_Developing_Safety-Critical_Code
- https://spinroot.com/gerard/pdf/P10.pdf 
- https://spinroot.com/static/index.html 
- https://web.eecs.umich.edu/~imarkov/10rules.pdf 
- https://en.wikipedia.org/wiki/Static_program_analysis 
- https://www.oreilly.com/library/view/designing-data-intensive-applications/9781491903063/
- https://www.oreilly.com/library/view/rust-atomics-and/9781098119430/
- Books by https://en.wikipedia.org/wiki/P._J._Plauger
#### mode_case_handling_framework_summary



# Mode & Case Handling Bullet Points Summary
## Framework: Three Modes & Rules

### Three Modes of Operation:
1. Production-Release Mode: 
- Never panics, halts, or leaks data.
- Uses 2-byte error codes (no heap, no strings, no PII).
- Smoothly handles all cases via "Let It Fail & Recover" (with optional logging).
- Uses a Three-Level Recovery Hierarchy 

2. Debug Mode:
- Similar to production but logs verbose diagnostics (heap allowed).
- Uses `eprintln!` for errors and `debug_assert!` for internal invariants (gated to avoid running in test/production).
- Optional debug printing/logging for inspection.

3. Test Mode:
- Uses `assert!` in test functions.
- Deliberately crashes/panics to generate stack traces for debugging.



### Rules:

1. All functions return `Result<T, YourProjectError>`. The error payload is always the 2-byte `YourProjectError` enum. All operations are expected to eventually fail in some way; all such failures must be smoothly handled.

2. Production-Release Mode (Result-Error case-handling):
- never panics
- no heap in error-code-return from functions
- no print or log of that code by the function before the code is returned

3. Separate Debug-Mode Assertions from Cargo-Tests:
- `debug_assert!` must be gated with `#[cfg(all(debug_assertions, not(test)))]` to exclude from test mode and production mode builds.

4. Use Gated Verbose Debug-Mode Diagnostics:
- Use `eprintln!` (gated with `#[cfg(debug_assertions)]`) for debug-only output. Heap is allowed/needed here.

5. Test-Mode Isolation:
- Test code must be gated with `#[cfg(test)]`. 
- Avoid using test-mode tests inside functions for stable code.

6. Use a Fieldless Enum error-code system.

7. Use Enforced-Custom-Types for Value-Integrity.

8. Use a Three-Level Recovery Hierarchy for recovery and state.


### Enforced-Custom-Types & Value-Integrity:
- Use `struct`/`enum`/`impl` to enforce value boundaries for inputs and intermediate values. This ensures invalid states (including bit-flips, corruption) are caught and handled as errors, not passed silently.
- required: add a validity-recheck into the .get() method
- Optionally use a .validity_recheck() method
- While a custom type in Rust can enforce and validate that a value is within the definition boundaries, this check only happens once when the constructor is run ( .new() ), assuming that the constructor was correctly written to carry out that check correctly. If memory-corruption happens after that initial check there are no automatic checks that will catch that the value is now invalid (e.g. as the value is returned from and accepted by another function). Additional .validity_recheck() methods can be made and used to manually check at specific points.
- Using custom types helps to manage compatibility within the design (e.g. narrowing scope of function inputs)
- vanilla custom types (without additional validity-checks) guard against design errors (coding-mistakes) and 'expected' errors.
- additional validity-rechecks can add guards against some electrical, hardware, and adversarial based errors, or 'unexpected errors.'
- Getting and mutation/updating use methods that must be manually designed to check & enforce boundary checks and validity.
#### Public & Private
- note: the individual rules for structs, enums, impls, and their combinations, vary.
- A struct CAN be safely pub (so other modules can use the type name in signatures).
- The struct's inner fields should NOT be pub (so other modules cannot bypass checks).
- Example of safety for combinations: if an enum variant carries data then do not put raw unbounded primitives inside that; rather, put a bounded struct inside.

### Three-Level Recovery Hierarchy ("Let It Fail & Recover")
Production-release failures should fit one of three bounded recovery tiers:
- Recovery Tier 1: Micro-Retry (Local) — If err.is_retryable() is true, a caller repeats the bounded operation (with backoff/sleep).
- Recovery Tier 2: Step Fallback / Safe Degradation (Subsystem) — This is at a level above which any retry-errors would be thrown. If non-retryable, or retry attempts are exhausted, the subsystem safely aborts the current command, and handles what to do next. This is highly case dependent, and might include trying another function, reverting to a previous state, exiting silently, etc. Some internal state may need to be reset. Ultimately move on with or without logging the case or "error."
- Recovery Tier 3: Macro Re-initialization — An outer loop reinitializes state and continues execution without halting. The largest case for Tier 3 reboots the entire program.

#### Recovery & State-Recovery:
Recovery tiers represent functional layers of 'state' in terms of 'recovery levels' to plan for, e.g. what to do when some state-values may not exist. Tier 1 covers the state in question. Tiers 2 and 3 range from small to large 'reboot/retry' scales up to the whole program. Full system restart will likely often have at least some state that it can be restarted with (resulting in no noticeable interruption), sometimes implying that state should be managed outside of retry-loops, to retain intact values.


### Error Handling System
- Single Fieldless Enum:
  A global `YourProjectError` enum (e.g., `#[repr(u16)]`) defines all error codes.
  Properties:
  - No heap (2-byte `Copy` values).
  - No `String` or `io::Error` payloads (prevents PII leaks).
  - Exhaustive `match` for retryability (`is_retryable()` method).
  - Append-only codes: Never reuse or renumber.

- Error Code Table Rules:
  1. Codes are unique and permanent (e.g., `Fs32tFromStrInputTooLong = 101`).
  2. Reserve blocks per module/feature (e.g., 100–199 for `FixedSize32Timestamp`).
  3. Variant names: `AcronymFunctionCondition` (e.g., `IoPermissionDenied`).
  4. Document codes in the enum’s doc comment.

- Display for Debug Only:
  `Display` impl (for human-readable text) is compiled only for debug/test builds.

#### Error Sites & Propagation:
- Unique codes permit use of '?' because the unique code of the root-cause error is preserved (not obscured) by propagating the error.
- If an error site is of note, that site needs to throw a specific error code.
- This acts as a proxy for manually checking for most internal-invariant type issues.


### Two Detection Patterns
1. If-Detection-Pattern:
   For direct condition checks (e.g., `if len > 32`).
   - Debug: `debug_assert!` (internal invariants only) + `eprintln!`.
   - Production: Return terse error code.

2. Match-A-Function-Call-Pattern:
   For handling `Result` from fallible calls (e.g., `std::str::from_utf8`).
   - Debug: `debug_assert!(false, ...)` (for internal invariants) + `eprintln!`.
   - Production: Drop callee’s error (to avoid heap/PII), return project error code.


## Key Principles
- No Heap in Production: Ban `String`, `format!`, `Box<dyn Error>`, etc.
- No Panics in Production: Use `checked_add`, `.get()`, etc., to avoid silent wraps/panics.
- Defensive Programming:
  - Input Validation: Expected issues (e.g., bad user input) → return error code + debug `eprintln!`.
  - Internal Invariant: "Should-not-happen" checks (e.g., bit-flips) → `debug_assert!` + production catch.
- Retry Logic: Defined per error code (not ranges) in `is_retryable()`.


### Example Code
#### Error Enum
```rust
#[repr(u16)]
pub enum YourProjectError {
    IoNotFound = 50,
    Fs32tFromStrInputTooLong = 101,
    RetryMaxAttemptsZero = 200,
}
```

#### If-Detection-Pattern
```rust
if !condition {
    #[cfg(debug_assertions)]
    eprintln!("ACRO-101: detail: {}", value);
    return Err(YourProjectError::AcroFnCondition);
}
```

#### Match-A-Function-Call-Pattern
```rust
match fallible_call(input) {
    Ok(v) => Ok(v),
    Err(_detail) => {
        #[cfg(debug_assertions)]
        eprintln!("ACRO-101: {}", _detail);
        Err(YourProjectError::AcroFnCondition)
    }
}
```

### Banned in Production
- `unwrap`/`expect`/`panic!`
- Heap allocations (`String`, `format!`, `Box<dyn Error>`)
- Error messages with PII (file/dir paths, user data, etc.)
- Unchecked Arithmetic: Arithmetic operators that can panic in production (use `checked_add`, etc.)


#### Suggestions (TODO: under construction; move to rules, for manageable use of codes?)
- Allow effective "blocks" of codes for functions, to allow coherent numbering and easy incrementing in case two developers collide. e.g. if the last two digits are internal to a function, u16 would allow for 654 functions to each have 99 internal errors (or depending on average function size, ten per function may be enough on average).
- alt phrasing: allow 10 or 100 error-codes per function to manage allocations and changes to codes



# Planning Notes Part 2: Specific automata-Design
```
# Granmo-Game Automata Theory → `ByteConvTM` Design Answers

Below: the four design questions answered against the 2018 paper's actual construction, then the concrete consequences for Phase 2, then resolutions for §11, then the rules-reconciliation needed before code.

---

## 1. What automaton properties does a Granmo-Game actually require?

In the 2018 construction, the learning-correctness of the system is **split in two**, and it matters enormously which half a property belongs to:

- **The payoff design (Type I / Type II feedback)** encodes the task. The theorem structure of the paper is: the feedback scheme is engineered so that Nash equilibria of the decentralized game coincide with clause configurations that optimize pattern recognition. **Task correctness lives here, not in the automaton.**
- **The automaton** is only required to be a *noise-filtering, bounded, local estimator* of "does the include action or the exclude action currently have the higher reward rate?"

Formalized, the automaton must satisfy five properties:

| # | Property | Why the game needs it |
|---|---|---|
| P1 | **Deterministic binary action from internal state** (include/exclude = sign side of a bounded counter) | Clause evaluation must be a pure function of state — enables integer inference, masks, and interpretability |
| P2 | **Bounded monotone state update**: reward moves deeper into the current action's region, penalty moves toward the boundary | This is the entire learning mechanism; any monotone saturating counter qualifies |
| P3 | **Ergodicity (no absorbing states)** | The environment seen by each automaton is **non-stationary** — every other automaton in the team is also learning, and clause roles migrate. An absorbing automaton would freeze the team in early, wrong equilibria. The clamp at `[1, 2N]` (not absorption at the ends) is load-bearing |
| P4 | **Sufficient state depth N** to filter stochastic feedback | Feedback arrives with probabilities `1/s`, `(s−1)/s`, gated by `(T∓V)/2T`, and (in the convolutional case) through *one sampled window*. Depth is the low-pass filter; too shallow ⇒ the team oscillates and never settles into an equilibrium |
| P5 | **Locality**: update depends only on own state + own reward/penalty event | This is what makes the game decentralized, O(1) per literal, and embarrassingly parallel in principle |

A subtle point worth recording: classical Tsetlin-automaton ε-optimality (P(correct action) → 1 as N → ∞) is proven for **stationary** environments and does **not** directly apply inside the team game. What the game actually relies on is P3+P4: enough inertia to resist noise, enough plasticity to escape bad joint configurations. This is why depth is a *tuning* parameter, not a correctness parameter.

## 2. Do the needed properties vary by game-outcome-task type?

Yes — P1/P2/P5 are universal, but **P3 and P4 requirements are task-dependent**, and the byte-convolution game shifts them measurably:

1. **Feedback noise rate differs.** With OR-pooling and single-sampled-window attribution, one Type Ia event touches one window out of potentially hundreds. Per-literal reward statistics are noisier than in flat BOW ⇒ the byte-conv game argues for **equal or deeper N**, not shallower.
2. **Feedback asymmetry differs per literal population.** At a given slot, exactly 1 of 256 positive literals is true in any window. The 255 "false" positive-literal automata live in an environment that is almost pure `1/s` decay pressure; the automaton for a genuinely predictive byte sees mixed traffic. These are *statistically different environments* attached to identical automata — see Q4.
3. **Stationarity regime differs by deployment mode.** Your offline train-test MVP tolerates deep, sluggish automata. Your backlogged **continual-stream on-the-fly training** version needs faster ergodic mixing (shallower N, or a deliberate leak) to track drift. Same game, different P3/P4 tuning — this justifies making `states_per_action` a first-class persisted config value, which the hand-off already does.
4. **Class imbalance** (expected in hate/bullying data) skews how often target vs non-target feedback fires per polarity, which again shifts effective penalty probabilities — another reason the automaton must merely be *robust* rather than *optimal-for-one-environment*.

## 3. Could a simpler or different automaton play the game?

Yes, and the literature-consistent answer is sharper than "yes": **the Tsetlin automaton already *is* the minimal deterministic automaton satisfying P1–P5 — it is exactly a saturating signed counter with a threshold.** Candidate substitutions, evaluated:

| Candidate | Verdict |
|---|---|
| **1-bit win-stay/lose-shift** | Fails P4 — no noise filtering; team never converges under stochastic `1/s` feedback |
| **L_RI / L_RP stochastic probability-updating automata** | Playable in principle, but state is a probability ⇒ fixed-point arithmetic, more memory, and loses the crisp deterministic include/exclude needed for masks and explanations. Strictly worse for this project's constraints |
| **Krinsky/Krylov variants** (stochastic penalty transitions) | Behaviorally ≈ TA with rescaled effective depth; no benefit here since `1/s` stochasticity is already injected at the feedback layer |
| **Shallower integer width (i8, N ≤ 127)** | Fully legitimate — depth is tuning (P4), not structure. Halves training memory (≈1.0 MB → ≈0.5 MB at K=5, 200 clauses). Recommend as a config option with i16 default and an M-Ablate arm |
| **Bounded counter + external integer clause weights (CoTM)** | Demonstrates part of the learning can be moved *out* of the automata entirely — confirms the automaton's role is replaceable/reducible as long as P1–P5 hold |

So: simpler in *width*, yes; simpler in *structure*, no — anything structurally simpler breaks P3 or P4.

## 4. Could a more optimal automaton be defined for the byte-convolution game specifically?

This is the interesting one. Three byte-specific structural facts create real optimization openings — two automaton-level, one encoding-level:

### 4a. The mutual-exclusion defect (strongest candidate)

At each slot k, the 256 positive literals are **categorically mutually exclusive**: a clause that includes two positive literals at one slot has an **empty allowed-bytes mask at that slot and can never fire anywhere**. The standard independent-TA team can and does enter these dead joint states. The game self-heals — a never-firing clause receives only Type Ib uniform decay until one literal drops out — but through a slow "drought" during which the clause contributes nothing.

**Proposed modified automaton: `GuardedInclude` TA.** Identical to the standard TA except the exclude→include boundary crossing for a *positive* literal is refused (state clamps at the boundary) whenever another positive literal at the same slot is already included. Assessment:

- Preserves P1–P4 exactly; relaxes P5 minimally (one read of the slot's cached include-count — a counter you must maintain anyway for mask upkeep, mirroring Phase 1's `included_positive_counts`).
- Eliminates only joint states that are *provably dead a priori* — it cannot remove any useful equilibrium.
- The fuller alternative — a single 257-action categorical/pursuit automaton per slot ("no constraint" ∪ "require byte b") — is theoretically cleaner but abandons the decentralized game structure the convergence argument rests on, and complicates Type II feedback. Not recommended for Phase 2.

**Recommendation:** implement standard TA as the recorded baseline, `GuardedInclude` as a config-flagged M-Ablate arm. Hypothesis to test: faster convergence (fewer epochs to plateau), equal-or-better final F1. This is precisely the kind of controlled single-variable experiment §8 demands.

### 4b. Asymmetric population tuning (cheap, low-risk)

The 255-per-slot "almost always false" positive-literal automata need include-side depth (stability once a byte is genuinely learned) but gain nothing from exclude-side depth — deep exclude states only delay rare, wrong inclusions. Negated literals ("byte b must not appear at slot k") receive extremely sparse per-byte feedback and tolerate shallow states. A packed representation — u8 per automaton, 7-bit state + implicit action bit, optionally different effective depths per literal class — quarters training memory versus i16 with no change to game semantics (depth is P4 tuning). Backlog candidate; do not complicate M-Conv-Core with it.

### 4c. Different game board, same automaton: bit-level literals (record in backlog)

Replacing 256 one-hot literals per slot with 8 *bit* literals (2×8×K = 80 literals/clause vs 2,560) lets clauses learn byte *classes* — and ASCII's bit layout is semantically aligned (bit 0x20 is the case bit; digits share a high-nibble). This is not a new automaton; it is a coarser propositionalization. Expected less precise for exact-pattern matching (the core hate-speech use case) but a legitimate ablation for the robustness suite. Backlog, after M-Ablate.

**Bottom line for Q4:** the automaton itself is near-minimal already; the exploitable slack in the byte-convolution game is (i) structurally dead joint states (4a), (ii) statistically heterogeneous automaton populations (4b), and (iii) the encoding granularity (4c) — not exotic replacement automata.

---

## §11 Open Decisions — Recommendations

1. **K = 5: confirm.** Covers the core of most English morphemes and slur stems; 2,560 literals/clause matches the footprint math in §4. Add a cheap K ∈ {4, 6} sweep to M-Ablate only if the K=5 plateau looks marginal.
2. **Stride 2 default: confirm**, S=1 as one M-Ablate run. One cost to record now: with S=2, a fixed byte pattern appears at *alternating in-window offsets* depending on parity of its document position, so the model needs roughly two clause variants per pattern. OR-pooling + 200 clauses absorbs this; the S=1 ablation quantifies exactly what it costs.
3. **Stemmer: drop stage 9 from Phase 2.** Your own prediction table rates it partially-redundant and typo-fragile; it is the only stage requiring either a dependency or nontrivial hand-rolled code, and it conflicts with the byte-stream-adapter purity of stages 1–8. Delete rather than stub — restoring it later is a clean additive change since the stage list is persisted config.
4. **Config UX: CLI flags only.** Interactive Q&A is incompatible with the fixed-seed, scripted, identical-split benchmarking discipline in §8. Every recorded run must be reproducible from a command line captured in a log.
5. **Weighted clauses: defer to backlog.** Do not confound M-Ablate; get the unweighted `ByteConvTM` result matrix first so a later CoTM arm has a clean baseline.
6. **Crate layout: split into modules.** The new case-handling rules effectively force this (see below).

## Rules Reconciliation (must be settled before M-Conv-Core code)

The supplied Mode & Case Handling rules **conflict with Phase 1's `PipelineError`** (String payloads, `format!`, heap in every error path). Proposed reconciliation, consistent with "frozen baselines are exempt":

- **New engine modules** (`byte_conv.rs`, `preprocess.rs`, `rng.rs`, and the packed-mask code) use a fieldless `#[repr(u16)]` `GranmoModelError` with block allocation (e.g., 100–199 preprocess, 200–299 RNG, 300–499 ByteConvTM, 500–599 artifact I/O), `is_retryable()`, `Display` gated to debug/test, `debug_assert!` gated with `#[cfg(all(debug_assertions, not(test)))]`. No heap, no PII, no panics in any engine path — including training.
- **The research harness** (CLI, dataset loading, `EvaluationReport`, the frozen `VanillaTM`/`WindowedTM`) stays on the existing `PipelineError`, with a single `From<GranmoModelError> for PipelineError` bridge at the boundary. The harness is a debug-mode research tool per the three-mode framework; the engine is the production artifact.
- Three-Level Recovery mapping for the engine: Tier 1 = none needed (no I/O in hot paths); Tier 2 = artifact-load validation failure ⇒ reject model, report code; Tier 3 = harness-level re-init (out of engine scope).
- Enforced-custom-types: `PatchSize`, `Stride`, `SpecificityThresholdU16`, `DecisionThreshold`, `ClauseCount` newtypes with validity-rechecking `.get()`, bounded at construction — replacing Phase 1's raw-primitive config fields for the new engine only.

## Proposed Immediate Sequence

1. **Confirm** the §11 recommendations above (one message from you suffices).
2. **M-Preprocess**: byte-stream adapter pipeline, stages 1–8, presets `raw`/P0–P5, persisted stage list — under the new error/type rules.
3. **M-ByteBag** in parallel: byte-n-gram view feeding the unchanged `VanillaTM`, first P0/P1/P2 runs.
4. **M-Conv-Core**: standard TA baseline first; `GuardedInclude` (4a) behind a config flag as the first automaton-level ablation arm; i8-depth option recorded but defaulted to i16 per the hand-off.

Say the word and I will start with the module skeleton: `error.rs` (full code-block allocation table) + `preprocess.rs` with its test suite.
```

# Task:
Please align on scope before any code, no code yet
Main priority for first discussion:  please start with focus on these questions:
Starting with:
4. For example, for a byte-convolution-game, could a more optimal Automata be defined?
5. Diverse Team Composition: Could ~teams of automata with diverse properties perform well for some game-spaces?

```
Question 5: Heterogeneous ("Diverse-Team") Automata Composition

Short answer: yes, theoretically safe and genuinely promising — and it costs almost nothing to leave the door open in today's code.

Why it is safe. Properties P1–P5 are per-automaton requirements, and the task-correctness lives in the payoff design (Type I/II feedback), not in the automata. Mixing automata that each individually satisfy P1–P5 does not change the payoff matrix and therefore does not change the set of good equilibria — it changes only the dynamics (convergence speed, noise sensitivity) and equilibrium selection (which good configuration the team lands in). That makes mixed teams a legitimate experiment rather than a gamble on correctness.

Why it is promising — the multi-timescale argument. A depth-heterogeneous team is a fast/slow-weights ensemble:

Shallow-N "scout" clauses: plastic, quickly capture new/rare byte patterns, noisy voters;
Deep-N "anchor" clauses: slow, stable holders of high-confidence patterns.

This is exactly the composition you would want for your backlogged continual-stream version (drift tracking + stable core vocabulary), and there is precedent for board-level heterogeneity working (multigranular CTM patch sizes; per-clause adaptive specificity in the TM literature).

Composition axes available to us, cheapest first: per-clause state depth N → per-clause specificity thresholds → per-clause GuardedInclude flag → (backlog) per-clause patch size K.

The real risk is experimental, not theoretical: (a) confounded ablations if introduced before the homogeneous baseline exists; (b) vote imbalance — late in training, deep clauses fire reliably while shallow clauses fire noisily, so raw ±1 voting may need per-clause integer weights to balance subpopulations, which interacts with the deferred CoTM arm.

Decision recorded: add M-Hetero as an M-Ablate arm after the homogeneous baseline; in today's foundations, the only accommodation is that specificity is represented as a per-instance thresholds value (not a global constant), so per-clause palettes later are an additive change, not a rewrite.
```



# SESSION 1 DELTA — Byte-Convolution Granmo Model, Sessions 1 (append to Phase 2 hand-off)

## 0. Terminology
"Granmo Model" per project convention. Crate: byte_conv_granmo_model (new,
flat single-file src/main.rs, ZERO dependencies, #![forbid(unsafe_code)]).
NOT code-shared with the Phase 1 crate; Phase 1 remains frozen reference.

## 1. §11 Open Decisions — ALL RESOLVED
1. K = 5 confirmed (bounds 2..=16; K∈{4,6} sweep optional in M-Ablate).
2. Stride 2 default confirmed; S=1 as ablation. Known recorded cost: with
   S=2 a fixed pattern appears at alternating in-window offsets by position
   parity (needs ~2 clause variants per pattern; quantified by S=1 ablation).
3. Stemmer (stage 9): DROPPED from Phase 2 entirely (not stubbed).
4. Config UX: CLI flags only (reproducibility; no interactive mode).
5. Weighted clauses (CoTM): deferred to backlog, after unweighted matrix.
6. Layout: flat single file, sectioned; new-engine code on the fieldless
   #[repr(u16)] error enum per the case-handling framework.

## 2. Automata design record (Q1–Q5, settled this session)
- Task-correctness lives in the Type I/II payoff design, NOT the automaton.
  Automaton needs properties P1–P5: deterministic binary action from state;
  bounded monotone update; ergodicity (no absorbing states — clamp [1,2N] is
  load-bearing); sufficient depth N (noise filter; TUNING not correctness);
  locality. Classical ε-optimality does not apply inside the team game.
- The Tsetlin automaton is already the minimal deterministic automaton
  satisfying P1–P5 (it IS a saturating signed counter). Exotic replacements
  rejected; depth-width reduction (i8) is a legitimate backlog memory win.
- Byte-specific defect: ≥2 positive includes at one slot ⇒ empty mask ⇒
  structurally dead clause (self-heals slowly via Type Ib drought).
  → `GuardedInclude` automaton variant IMPLEMENTED behind config flag
  (refuses exclude→include crossing for a positive literal when another is
  already included at that slot). DEFAULT OFF. It is an M-Ablate arm.
- Heterogeneous teams (Q5): safe to diversify P4-tier properties (depth,
  specificity, guard) per clause; unsafe to diversify P1/P2/P5. Accommodated
  now only by keeping specificity as per-instance thresholds (not a global).
  M-Hetero = ablation arm AFTER homogeneous baseline. Backlog: bit-level
  literals (8 bits/slot) as an encoding ablation; asymmetric/packed u8
  automata as a memory optimization.

## 3. Code inventory (src/main.rs sections; ALL tests passing)
S1 GranmoModelError: fieldless u16, blocks: 100 Pp / 200 Rng / 300 Cfg /
   400 Bct / 500 Art / 600 Prb / 700 Ds / 800 Cli. Append-only. Retryable:
   ONLY ArtFileWriteFailed(501), ArtFileReadFailed(502), DsFileReadFailed(701).
   Display gated to debug/test. debug_assert unused so far; eprintln
   diagnostics gated #[cfg(debug_assertions)].
S2 FastRng: xorshift64; next_u16 (top bits); coin(u16 threshold);
   gen_index via u128 widening multiply (bias < len/2^64, documented).
S3 Enforced newtypes (private fields, bounds at construction, revalidating
   .get()): PatchSize(2..=16), StrideLen(1..=8, ≤K checked at engine),
   ClauseCount(even, ≤65534), StatesPerAction(8..=16000, i16),
   SpecificityThresholds(u16 pair, sum==65536; from_specificity(f64) at
   config time ONLY; from_raw_thresholds for float-free artifact load),
   VoteThreshold(1..=10000), MaxScanBytes(16..=1_048_576).
S4 BytePreprocessor: stages 1–8 ALL map-or-drop ⇒ process_byte: u8→Option<u8>,
   two bits of state, streaming == batch (tested). PreprocessProfile = u16
   bitmask, presets raw/P0–P5. IMPORTANT SEMANTIC: stage order is canonical;
   in P3 leet-fold(5) runs BEFORE symbol-strip(7), so carriers !,@,$,0,1,3
   always fold and survive stripping (intentional, tested). A strip-carriers
   profile = P0+stages6+7 without leet (expressible via from_bits; unnamed).
S5 ByteConvTM: binary single bank (even=+, odd=−); literals (k,b) pos at
   k*256+b, neg at K*256+k*256+b; i16 states in [1,2N]; per-(clause,slot)
   [u64;4] allowed-bytes masks recomputed ONLY on boundary crossings; all
   state changes route through increment/decrement_ta_state; OR-pooling with
   predict early-exit; training scan = single pass with reservoir-sampled
   fired window (no fired-Vec); integer feedback gates (draw r∈[0,2T) vs
   T∓V); Type Ia/Ib/II per Phase 1 semantics; PAD 0x00 right-pad when
   doc<K; max_scan cap; validate_internal_consistency (band + counts +
   masks); fired_window_positions, describe_clause, fired_clause_bits
   (embedding deliverable). CountFire pooling NOT implemented (M-Ablate).
S6 run_self_check: preprocess + fresh-engine smoke; main exits with error
   code on failure.
S7 ProbabilityLut: sigmoid(V/(T/2)) filled at config time (only float
   site), runtime = one u16 index; validity_recheck (monotone).
   sweep_decision_thresholds: full integer threshold sweep, confusion
   counts + P/R/F1 per row (floats = offline reporting, permitted).
S8 ModelArtifact (FULL training artifact, kind=1): magic GRANMOB1, version 1,
   header (profile bits, K, S, clauses, T, N, forget/reinforce u16s,
   max_scan, guard, kind) + raw i16 states + FNV-1a-64 trailer. Masks/counts
   NOT stored — rebuilt from states on load, then
   validate_internal_consistency as final gate. Round-trip is behaviorally
   bit-exact (tested). Kind=2 CompactInference RESERVED for M-Prod-Pass
   (will embed LUT verbatim to keep production load float-free).
S9 Dataset: dependency-free RFC-4180-subset CSV parser (quotes, escaped
   quotes, embedded commas/newlines — required for social-media text);
   load_labeled_csv (binary label = byte-equality with --positive-label,
   all else negative; empty cells skipped); split_dataset (integer percent
   1..=99, seeded Fisher–Yates, deterministic; tested). JSONL: deferred
   (backlog — no dependency-free JSON parser this phase).
S10 run_single_experiment: preprocess-once → per-epoch seeded shuffle →
   train → eval (accuracy@V>0 + full sweep + best-F1 row + timing).
S11 CLI: modes train/batch/predict; batch = presets raw,p0,p1,p2 on ONE
   identical split+seed. predict replays the persisted profile, prints vote,
   LUT probability, and fired-clause explanations with byte offsets into the
   PREPROCESSED stream. Fail-fast parsing. Defaults: K5 S2 200 clauses T50
   N100 s5.0 max-scan1024 epochs25 seed42 split80.

## 4. Known caveats / tuning notes
- Engine learning test (negation micro-corpus) passes at: K5 S1, 32 clauses,
  T15, N100, s3.0, 300 epochs, seed 42. Harness e2e test passes with same +
  shuffling, 400 epochs, tolerance ≥0.875 accuracy / ≥0.85 F1.
- Corruption test flips byte 40 — must stay within 32+2·state_count.
- Always use --release for real datasets.
- train_step allocates one scratch Vec per step (documented, harness-tier);
  inference paths allocate nothing.
- Explainability offsets refer to preprocessed bytes; mapping back to raw
  source offsets is future work (needs per-stage offset tracking).

## 5. Milestone status → next actions (priority order)
DONE: M-Preprocess, M-Conv-Core, M-Explain (primitives), M-Prob, full
artifact I/O, harness/CLI.
NEXT (a) M-ByteBag: flat byte-n-gram baseline — THE scientific control for
  "does convolution earn its complexity" (§8 success criteria depend on it).
NEXT (b) M-Prod-Pass: compact artifact kind=2 (masks+polarities+LUT+profile,
  drop states), size/throughput measurement.
NEXT (c) Robustness suites: obfuscation generator + word-boundary/negation
  minimal pairs (build once, freeze).
THEN M-Ablate matrix incl.: OR vs CountFire, S1 vs S2, GuardedInclude,
  i8-depth, M-Hetero (mixed depth/s), byte-bit-literal encoding; CoTM
  weights last.


# Session 2 Alignment — Q4, Q5, and M-ByteBag Scope 

## 1. Q4: A more optimal automaton for the byte-convolution game — position confirmed, one addition

The Session-1 position stands and nothing in it needs reopening:

- The Tsetlin automaton is structurally minimal for P1–P5; the exploitable slack is (i) dead joint states, (ii) heterogeneous literal populations, (iii) encoding granularity — not replacement automata.
- `GuardedInclude` is implemented, default OFF, an M-Ablate arm. Correct placement: it prunes only a-priori-dead configurations, so it cannot remove useful equilibria, but it must not contaminate the recorded baseline.
- 4b (packed u8 / asymmetric depth) and 4c (bit-level literals) remain backlog. Confirmed.

**New observation worth recording before M-ByteBag:** the flat byte-n-gram baseline is *itself* an implicit control on the mutual-exclusion defect. A bag encoding has no categorical per-slot structure, hence **no structurally dead joint states exist for it at all**. This gives us a three-way diagnostic once the matrix runs:

| Outcome pattern | Interpretation |
|---|---|
| Conv(no guard) ≈ Conv(guard) ≈ ByteBag | Defect immaterial; convolution not earning complexity |
| Conv(no guard) < ByteBag, Conv(guard) ≈ ByteBag | Defect is real and the guard repairs it |
| Conv(guard) > ByteBag | Convolution earns its complexity independent of the defect |

This costs nothing — it falls out of runs we were doing anyway — but it should be written into the M-Ablate analysis plan now so the comparison is read correctly later. Please treat this as an addition to the §8 analysis notes.

## 2. Q5: Diverse-team composition — recorded decision confirmed, two small deltas

The quoted decision block is sound and I confirm it as recorded: mixing automata that individually satisfy P1–P5 changes dynamics and equilibrium selection, not the equilibrium set; M-Hetero is an ablation arm strictly after the homogeneous baseline; the only present-day accommodation is per-instance (not global) specificity thresholds — which Section 3 of the current code already satisfies (`SpecificityThresholds` is a per-engine value).

Two deltas to record:

1. **The accommodation must also hold in the ByteBag engine.** When we write it, specificity/depth are per-instance fields exactly as in `ByteConvTM`, so M-Hetero later applies to both engines without rewrite. Trivial, but worth locking now.
2. **Cheap instrumentation for the vote-imbalance risk.** The identified risk (deep "anchor" clauses fire reliably, shallow "scouts" fire noisily, distorting raw ±1 voting) is directly measurable from data we already compute: the per-clause fire-rate histogram over the test set is a fold over `fired_clause_bits`. Recommend adding a fire-rate report to the harness during M-Ablate (not now) as the diagnostic that decides whether M-Hetero needs the CoTM weight arm pulled forward. No foundation change required today.

## 3. Scope decision required for (a): where M-ByteBag lives

This is the one genuine scope question before code. The original hand-off said "tokenizer swap on the unchanged `VanillaTM`" — but that plan predates the Session-1 pivot to a new zero-dependency crate. `VanillaTM` lives in the frozen Phase 1 crate with a *different* CSV parser, RNG, split routine, and preprocessing path. Running the control there would violate the identical-splits/identical-preprocessing discipline that the entire §8 comparison rests on, or force us to prove cross-crate split equivalence — worse than just writing the baseline.

**Recommendation: implement `ByteBagTM` in this crate.** It shares `BytePreprocessor`, `load_labeled_csv`, `split_dataset`, `FastRng`, `sweep_decision_thresholds`, and the report printer — so the control and the treatment differ in exactly one variable (flat bag vs. convolution), which is the point of a control.

Proposed specification (for your confirmation, no code yet):

- **Feature extraction:** overlapping byte shingles of fixed length `n`, **stride 1** (shingling stride is not a tunable here — stride is a convolution concept; a bag wants full coverage). Documents shorter than `n`: right-pad 0x00, one shingle (consistent with locked PAD decision).
- **Vocabulary:** top-`M` most frequent shingles over the *training* split only; deterministic ordering (count descending, then bytewise-ascending tiebreak) so identical seeds give identical vocabularies. Shingles outside the vocabulary are simply absent features — no OOV token (this preserves the "no OOV concept" comparison honestly: the bag *does* have coverage loss, the conv model doesn't; that difference is part of what we're measuring).
- **Engine:** flat clause bank, same even/odd polarity scheme, literal `j` = "shingle j present" plus negated twin (2×M literals per clause); presence bits, not counts. Type Ia/Ib/II per Phase 1 flat semantics — no window sampling (no positional structure exists). Same integer gate `(T∓V)/2T`, same `SpecificityThresholds` coins, same `[1,2N]` band routed through increment/decrement methods, same consistency validation pattern. Evaluation via a per-clause included-literal list (flat engines are sparse-include; a 256-bit mask has no analogue here).
- **Defaults for comparability:** `n = 5` (equal to K, so both models see the same maximal pattern length) and `M = 4000` (matches Phase 1's word-vocabulary scale, making the "10× literal-space" argument auditable: bag = 8,000 literals/clause vs. conv 2,560). Optional `n ∈ {3,4}` sweep deferred to M-Ablate.
- **Artifact:** ByteBag must persist its **vocabulary** (it is learned state). Fixed `n` means fixed-width serialization: `M × n` bytes, no length prefixes. Propose artifact kind byte = 3 under the existing magic/checksum framing, states + vocabulary in the payload, same four-gate load validation.
- **Error codes:** new append-only block **900–999 `Bbg*`** (800 is CLI; 900 is next free).
- **CLI:** add `--engine byte-conv|byte-bag` (default `byte-conv`), `--ngram-len 5`, `--vocab-size 4000`. Batch mode gains the ability to run both engines on the one split — the comparison-matrix command.
- **Tests:** replicate the established style — vocabulary determinism, presence semantics, order-*insensitivity* (the mirror of the conv order-sensitivity test: `"zab"` vs `"zba"` must be indistinguishable to the bag — that asymmetry is the scientific contrast), padding, invariants after training, artifact round-trip, learning test on the same negation micro-corpus (with a recorded prediction: the bag should *also* pass it, since "not g" vs "very " are themselves distinguishing shingles — the suites where the bag should lose are the word-boundary/positional minimal pairs in NEXT (c)).

## 4. Confirmations needed (one line each suffices)

1. ByteBagTM in this crate per §3 above (vs. Phase 1 crate) — confirm.
2. Defaults `n = 5`, `M = 4000` — confirm or adjust.
3. Error block 900–999, artifact kind 3 — confirm.
4. Record the §1 three-way diagnostic table into the M-Ablate analysis plan — confirm.

## 5. Anything else before session 2?

Nothing blocking. One non-blocking item to have in hand soon: **the actual dataset file** (path, column names, positive-label value) — the harness is dataset-ready, and the first real M-ByteBag vs. ByteConvTM comparison run needs it. Not required to write the ByteBag code.

On your confirmation of §4, the next message will be the M-ByteBag implementation: Section 12 (`ByteBagVocabulary` + `ByteBagTM`), the 900-block error variants, artifact kind 3, CLI extensions, and the test suite — under all existing rules (fieldless error enum, enforced newtypes with revalidating `.get()`, integer-only hot paths, no-panic, checked arithmetic, append-only codes).



Handoff Doc for Session 3: 

# SESSION 2 DELTA — Byte-Convolution Granmo Model (append to Phase 2 hand-off + Session 1 delta)

Carry into the new session: **Phase 2 hand-off doc + Session 1 delta + THIS document + `src/main.rs` + `Cargo.toml` + the Rust rules & Mode/Case-Handling framework.** All prior locked decisions stand except where explicitly superseded below.

---

## 0. Terminology and Ground Rules (unchanged, restated for the new session)

- "Granmo Model" per project convention (never "Tsetlin machine"; "TM/CTM" only if unavoidable).
- Crate: `byte_convolusion_series_teamgames`, flat single-file `src/main.rs`, **zero third-party dependencies** (hard requirement, reaffirmed this session), `#![forbid(unsafe_code)]`.
- Mode & Case Handling framework binds all new code: fieldless `#[repr(u16)]` `GranmoModelError`, append-only codes, no heap/no PII/no panic in error paths, `Display` gated debug/test, `eprintln!` diagnostics gated `#[cfg(debug_assertions)]`, enforced newtypes with revalidating `.get()`, checked arithmetic everywhere, `?` only where the root code is preserved, absolute paths only, cargo tests replicate established styles.
- Integer-only hot paths for engines; floats only at config time (`SpecificityThresholds::from_specificity`, LUT fill) and offline reporting.

## 1. Decisions Made This Session (locked)

| # | Decision | Value |
|---|---|---|
| S2-1 | **Dataset format** | **JSONL ONLY.** One JSON object per line; fields `"text"` (JSON string) and `"label"` (string or number). CSV support fully REMOVED (unfit for unstructured social-media text). Error codes 702, 703, 704, 713 are **retired — numbers permanently reserved, never reuse**. |
| S2-2 | Model persistence | Binary artifact only (magic + header + payload + FNV-1a-64 checksum). No JSON model files. Kind 1 = full training (implemented); kind 2 = compact inference (reserved, M-Prod-Pass); **kind 3 = ByteBag (reserved, next drop)**. |
| S2-3 | `ByteBagTM` lives in THIS crate | Not in the frozen Phase 1 crate — it must share the preprocessor, loader, splitter, RNG, and sweep so the §8 comparison differs in exactly one variable (positional windowing). |
| S2-4 | ByteBag defaults | n-gram length **n = 5** (= K, same maximal pattern length as conv), vocab size **M = 4000** (Phase 1 word-vocab scale; literal audit: bag 2×M = 8,000 vs. conv 2×K×256 = 2,560). |
| S2-5 | ByteBag structure decisions | Shingles are **overlapping, stride 1** (not tunable — stride is a convolution concept). Docs shorter than n → ONE right-0x00-padded shingle (PAD rule §10.4). Vocabulary = top-M shingles of the TRAINING split, ordered **count descending, bytes ascending tiebreak** (deterministic, machine-independent). **No OOV token** — out-of-vocab shingles are absent features; the bag's coverage loss vs. conv's total coverage is part of what the experiment measures. Presence bits, not counts. |
| S2-6 | Error blocks | Cfg 315–318 (ByteBag newtypes), **Bbg 900–999** (ByteBag module), Ds 708–712 (JSONL). |
| S2-7 | Three-way diagnostic recorded into M-Ablate analysis plan | Conv(no-guard) ≈ Conv(guard) ≈ Bag ⇒ mutual-exclusion defect immaterial AND conv not earning complexity. Conv(no-guard) < Bag but Conv(guard) ≈ Bag ⇒ defect real, guard repairs it. Conv(guard) > Bag ⇒ convolution earns complexity independent of the defect. (Bag has no per-slot categorical structure ⇒ no structurally dead joint states ⇒ it is an implicit control on the GuardedInclude hypothesis, free.) |
| S2-8 | Fire-rate report approved | Per-clause fire-frequency histogram over the test set (fold over `fired_clause_bits`) — the M-Hetero vote-imbalance diagnostic. Scheduled for Drop 2.2c, harness reporting tier. |
| S2-9 | Q5 heterogeneous teams | Confirmed as recorded (Session 1): safe to diversify P4-tier properties per clause; M-Hetero strictly after homogeneous baselines of BOTH engines; only present accommodation = per-instance specificity/depth (already satisfied in both engines' designs). |
| S2-10 | Communication protocol | Small drops; every drop states explicitly what is ADDED / REPLACED / DELETED. No "optional" edits. |

## 2. Code Delta This Session (all tests passing: 54)

**S9 REWORKED — JSONL-only ingestion:**
- REMOVED: `parse_csv_bytes`, `load_labeled_csv`, `trim_ascii_edges`, format dispatcher, all CSV tests, CLI flags `--text-col`/`--label-col`.
- ADDED (Section 9B): dependency-free JSONL record extractor — NOT a general JSON parser. `JsonLineCursor`, `decode_json_string` (full escape set incl. **\uXXXX surrogate pairs → UTF-8 bytes**; raw UTF-8 passes through per §10.5; lenient on raw control bytes — preprocessor's jurisdiction), `skip_json_value` (iterative depth-tracked skipper, escape-aware in strings — unknown keys tolerated), `capture_bare_token` (unquoted labels), `parse_jsonl_record`, `load_labeled_jsonl`.
- **Skip-vs-error policy (of record):** SKIP silently = blank line, missing text/label key, empty text, non-string text, `null` label, empty object. HARD ERROR = structural malformation on any line (708–712; the file is not what the operator thinks). Line-number diagnostics gated to debug builds (line numbers are not PII; content never printed).
- Label semantics: decoded token bytes compared byte-for-byte to `--positive-label` (default `"1"`); `"label":1` ≡ `"label":"1"`; `1.0` ≠ `1` by design (loader is float-free, no numeric parsing). Duplicate keys: last wins (tested).
- CLI: `--text-key` / `--label-key` (defaults `text`/`label`) replace the column flags; help text rewritten for JSONL-only.
- **Test-boundary note (bug found & pinned):** 709 fires only when a line ends INSIDE a string; a terminated string followed by garbage is 712. Both sides of the boundary are now pinned by tests.

**S12 (part 1) ADDED — `ByteBagVocabulary` (M-ByteBag foundation):**
- Newtypes: `NgramLength` (2..=16, default 5 — deliberately distinct type from `PatchSize`, never interchangeable in signatures), `VocabSize` (2..=65000, default 4000). Codes Cfg 315–318.
- Storage: `ngram_flat_bytes` rank-ordered flat (rank = feature index; fixed-width — this exact form is what artifact kind 3 persists, no length prefixes), `lookup_order` = ranks byte-sorted for binary-search lookup — **derived data, rebuilt on load, never persisted/trusted** (same pattern as conv masks).
- `build_from_documents` (HashMap counting is fine — determinism comes from the full sort afterwards; actual vocab may be < M, recorded not error; empty corpus = Bbg 900), `lookup` (Ok(None) = absent feature; wrong-length shingle = wiring error 902), `ngram_at_rank` (explainability primitive), `extract_presence_bits` (scan-capped, padded-short-doc path), `validity_recheck` (flat-length divisibility, strict permutation + strict byte ordering — strictness doubles as duplicate detection).
- Tests: ranking/tiebreak, truncation + miss semantics, determinism, PAD path, presence-bit exactness + scan cap, **order-insensitivity** (`"abab"` vs `"baba"` identical bits — THE scientific contrast with the conv engine's order-sensitivity test), empty-corpus rejection, newtype bounds.

## 3. NEXT ACTION — Drop 2.2b: `ByteBagTM` flat engine (specification agreed, implement first)

Section 12 continuation. Everything below was aligned this session; implement without re-deriving:

- **Structure:** single flat clause bank, even indices = +1 polarity, odd = −1 (same scheme as conv). Per clause, `2 × M_actual` automata: literal `r` = "vocabulary shingle r present", negated twin = "absent". i16 states in `[1, 2N]`, include iff state > N, ALL changes through increment/decrement methods (no mask analogue — flat engines evaluate via included-literal sets; keep a cached per-clause include list or count structure and validate it in a `validate_internal_consistency` mirroring the conv pattern).
- **Evaluation:** clause fires iff every included positive literal's bit is set AND every included negated literal's bit is clear in the document's presence bitset. Input to eval/train = `extract_presence_bits` output (extract ONCE per document per step).
- **Feedback:** Phase-1 FLAT semantics — **no window sampling, no reservoir** (no positional structure exists). Type Ia over the full literal vector against the presence bits (true literals reinforce at `reinforce_threshold` coin, false decay at `forget_threshold` coin); Type Ib input-independent 1/s decay; Type II increments every false literal (deterministic). Same integer vote gate: clamp V to ±T, draw r ∈ [0,2T), compare vs T∓V. Reuse `VoteThreshold`, `StatesPerAction`, `SpecificityThresholds`, `MaxScanBytes`, `ClauseCount`, `FastRng` unchanged.
- **Constructor takes the built vocabulary** (vocabulary is built from the preprocessed TRAINING split only — never test data; the harness must enforce this ordering).
- Public surface mirrors conv: `vote_sum`, `predict`, `train_step`, `fired_clause_bits`, `describe_clause` (decode literal ranks via `ngram_at_rank` — render printable bytes as chars, else hex, same convention), `validate_internal_consistency`, `clause_count`.
- Error codes: continue Bbg block (904+, append-only) for engine-specific conditions.
- **Tests to replicate:** order-INsensitivity at the engine level (mirror of conv's order-sensitivity test), invariants-after-stochastic-training (including empty/short docs), learning test on the SAME negation micro-corpus (recorded prediction: the bag SHOULD pass it — "not g" vs "very " are distinguishing shingles; the bag should lose only on the positional/word-boundary minimal-pair suites, NEXT item 5), describe_clause decode, guarded flag does NOT apply (no slot exclusivity — do not add it).

## 4. THEN — Drop 2.2c (agreed content)

1. **Artifact kind 3:** vocabulary flat bytes (`M_actual × n`, fixed width) + engine header + raw i16 states, same magic/checksum framing, four-gate load (checksum → enforced-type header parse → derived-cache rebuild incl. `rebuild_lookup_order` → full consistency validation). Behavioral round-trip test.
2. **CLI:** `--engine byte-conv|byte-bag` (default byte-conv), `--ngram-len 5`, `--vocab-size 4000`; batch mode gains both-engines-on-one-split (the comparison-matrix command).
3. **Fire-rate report** (S2-8) in `ExperimentReport` for both engines.

## 5. Remaining Roadmap (priority order, unchanged otherwise)

1. **M-Prod-Pass:** compact artifact kind 2 (conv masks + polarities + LUT verbatim + profile header; drop TA states), float-free production load, size/throughput measurement (tens-of-KB deliverable).
2. **Robustness suites (build once, freeze):** obfuscation generator (leet/symbol/casing rewrites of held-out positives) + word-boundary/negation minimal pairs — the suites where conv is predicted to beat the bag.
3. **M-Ablate matrix** on identical splits/seeds: raw/P0/P1/P2 presets × {ByteConvTM, ByteBagTM}; then OR vs CountFire, S=1 vs S=2, GuardedInclude (read via the S2-7 three-way table), i8-depth, M-Hetero (mixed depth/s, informed by fire-rate report), byte-bit-literal encoding; CoTM weights last.
4. §8 success criteria unchanged: (a) no macro-F1 regression vs. flat baseline, (b) clear obfuscation-suite win, (c) production artifact profile. Bag ≈ conv everywhere is itself a valid finding.

## 6. Known Caveats / Tuning Notes (carry forward)

- Engine learning test passes at K5 S1, 32 clauses, T15, N100, s3.0, 300 epochs, seed 42; harness e2e at 400 epochs, tolerance ≥0.875 acc / ≥0.85 F1. If stochastic-outcome tests fail on new hardware, first knobs: epochs +, clauses + — tune against observed votes, do not weaken invariant tests.
- Artifact corruption test flips byte 40; must stay within `32 + 2×state_count` — keep test engines large enough.
- `--release` for any real dataset. `train_step` allocates one scratch Vec (documented, harness tier); inference paths allocate nothing.
- Explainability offsets refer to PREPROCESSED bytes; raw-source offset mapping is future work (per-stage offset tracking).
- Vocabulary must be built from preprocessed training-split docs only — leakage guard is the harness's responsibility (test it in 2.2b/c).
- Backlog unchanged: GraphTM-lite depth-2, CoTM weights, caps-sentinel preprocessing, packed-u8/asymmetric automata, bit-level literals, Platt LUT refill, stream evaluator with paired offset windows, multi-class.

**First concrete action for the new session: implement Drop 2.2b exactly per §3, small-chunk protocol per S2-10.**


...

Drop 2.2c-ii — Engine Selection, Comparison-Matrix Batch Mode, Fire-Rate Report

Purpose: (1) --engine byte-conv|byte-bag selection with the bag hyperparameter flags --ngram-len / --vocab-size; (2) engine-construction dispatch in run_single_experiment, including the vocabulary leakage guard (ByteBag vocabulary built from the preprocessed TRAINING split only — the harness responsibility recorded in Session 2 §6, now implemented and tested); (3) batch mode becomes the §8 comparison-matrix command (presets raw/p0/p1/p2 × both engines on one identical split and seed); (4) the S2-8 fire-rate report for both engines (per-clause fire counts over the test set — the M-Hetero vote-imbalance diagnostic).

Evaluation-pass design note (behavior-preserving): the harness now computes ONE fired-clause bitset per test document and derives from it both the vote sum and the fire counts. The derived vote is definitionally identical to the engine's vote_sum (both apply the crate-wide polarity rule — even clause +1, odd −1 — to the same per-clause firing results), so all recorded accuracies and the existing test tolerances are unchanged. The training path and RNG stream are untouched.

Naming compliance notes: config fields for the bag are named bag_ngram_len and bag_vocab_size — NOT ngram_length/ngram_len, which already name a ByteBagVocabulary method and field respectively. New names verified unique: EngineSelection, engine_selection_from_name, vote_from_fired_words, summarize_fire_counts, engine_name_reported, clause_fire_counts, CliUnknownEngine, CliFireRateReportInternalFault.

...

# SESSION 3 DELTA — Handoff (append to Phase 2 hand-off + Session 1 & 2 deltas)

Carry into Session 4: **all prior hand-off docs + this delta + `src/main.rs` + `Cargo.toml` + the Rust rules & Mode/Case-Handling framework.**

**0. Ground rules (additions).** (a) NAMING LAW: one name = one thing, crate-wide. Engine methods are prefixed (`conv_`, `bag_`); plain dispatch names exist only on `ClassifierEngine`; field/local names avoid cross-entity reuse. (b) EDIT PROTOCOL: whole-block ADD/REPLACE on globally unique anchors, or exact-line edits with location qualifiers; never ambiguous references; transient context lives in removable `DROP NOTE` code comments, permanent semantics in doc comments.

**1. Decisions locked this session.** S3-1: `ClassifierEngine` enum (not trait objects) is the polymorphic seam; harness/artifact/CLI written against it once. S3-2: artifact format v2 — kind byte in common prelude (offset 10); kinds: 1=ByteConv full, 2=compact inference (RESERVED, M-Prod-Pass), 3=ByteBag full (vocabulary flat bytes + states); v1 rejected at version gate. S3-3: per-clause P4 accessors (`depth_for_clause` etc., both engines) are the sole present M-Hetero accommodation; `TeamCompositionPalette` is explicit-palette-at-engine-level, scalar-diversity-knob lives in the future training-coach as a palette generator; nothing more before M-Hetero. S3-4: shared integer gate helper `resolve_feedback_gates` (Trn block 1000) used by both engines. S3-5: ByteBag per Session-2 spec — flat feedback, no guard, vocabulary-leakage guard implemented in `run_single_experiment` (train split only; tested). S3-6: fire-rate report (S2-8) implemented; single evaluation pass derives vote + fire counts from one fired-bitset per test document (`vote_from_fired_words` equivalence documented). S3-7: batch mode = comparison matrix (raw/p0/p1/p2 × both engines, one split/seed).

**2. Code delta.** Drops 3.0a (enum + v2 artifact), 3.0b (accessors, gate helper, `shuffle_in_place`), 2.2b (`ByteBagTM`, Section 12B, `bag_` names, codes 904–907), 2.2c-i (ByteBag variant wiring + kind 3, `from_flat_bytes`, `bag_rebuild_masks_from_states`), 2.2c-ii (`EngineSelection`, `--engine/--ngram-len/--vocab-size`, codes 806–807, fire-rate), 3.1 (rename cleanup). Error blocks now: 100 Pp / 200 Rng / 300–318 Cfg / 400 Bct / 500–509 Art / 600 Prb / 700 Ds (702–704, 713 retired) / 800–807 Cli / 900–907 Bbg / 1000 Trn. **70 tests passing.**

**3. Stochastic-outcome tests (3):** `learns_byte_level_negation_micro_corpus`, `bag_learns_negation_micro_corpus`, `harness_bag_engine_end_to_end_on_negation_corpus` (+ the conv harness e2e with tolerances). Knobs if hardware-divergent: epochs +, clauses +; tune against observed votes, never weaken invariant tests. Corruption test flips byte 40 (must stay within 32 + 2×state_count). Always `--release` on real data.

**4. Next actions (priority).** (1) Robustness suites (build once, freeze): obfuscation generator + word-boundary/negation minimal pairs — where conv is predicted to beat bag. (2) M-Prod-Pass: kind-2 compact artifact (conv masks + polarities + LUT verbatim + profile; drop states), float-free load, size/throughput. (3) M-Ablate matrix on identical splits: read via the S2-7 three-way diagnostic (conv±guard vs bag); OR vs CountFire, S1 vs S2, GuardedInclude, i8 depth, K/n sweeps; fire-rate report gates whether M-Hetero needs CoTM weights pulled forward. (4) M-Hetero via `TeamCompositionPalette` (artifact v3/new kind). CoTM weights last.

(possible update:4. Team diversity: concrete plan
The palette-not-scalar decision stands as recorded. Now that it's unblocked, the minimal first experiment:
Engine: ByteBag first (it is the engine that matters now; conv gets the palette later via the same seam — both engines' depth_for_clause/*_threshold_for_clause accessors were built for exactly this, and the parallel path snapshots properties per clause already, so the seam survived parallelization intact).
Palette v1: two species only — anchors (deep N, high s) and scouts (shallow N, low s) — each species a (clause_fraction, StatesPerAction, SpecificityThresholds) tuple. No guard field for bag (recorded: guard doesn't apply). Deterministic assignment by contiguous rank blocks within each polarity so +/− banks stay balanced per species.
Artifact: new kind (4 = ByteBag-heterogeneous) rather than mutating kind 3 — keeps kind 3 loads byte-stable and avoids a format-version bump. Homogeneous = single-species palette, so kind 3 remains the degenerate case and current behavior is preserved exactly.
Diagnostic: per-species fire-rate histograms (scope E) — this is what decides whether the recorded vote-imbalance risk is real and whether CoTM weights get pulled forward.
First recorded run: homogeneous bag (current champion config) vs. 70/30 anchor/scout on the same split — one variable, one row, read against the fire-rate report.
The coach-side scalar-diversity→palette generator remains future scope (training-coach), as recorded.

Open parameter I need your ruling on: anchor/scout starting values. My proposal: anchors N=200/s=5.0, scouts N=25/s=2.5, fraction 0.7/0.3 — deliberately wide separation so the first run is diagnostic rather than subtle.)
(## The full-scale result closes the guard question — and opens a specialization one

`always 44/600 (44 vacuous, 0 specialized)` — confirmed: **zero pathological always-firers exist at full scale**. The guard finding can be recorded as final (guard validated, target-free, retained as ablation arm, conv port deprioritized).

But look at the histogram: `min 0  p25 1  median 1  p75 235  max 290`. At N=200/12 epochs, **half the bank holds ≤1 included literal** — starkly bimodal versus the N=50 run's healthy `median 52`. Roughly 300 clauses are heavily specialized (235–290 includes) and ~300 are barely specialized or vacuous. That's the depth-vs-epoch-budget diagnosis confirmed quantitatively: at N=200 an automaton needs 200 net reinforcements to cross the boundary, and the (T∓V)/2T gate starves half the bank before it gets there. Cheap follow-up when you want it: `batch-guard --guard-limits 0` with `--states 100` and `--states 50` at otherwise-full settings (~7 min each) — if F1 holds or improves while median includes rises, N=200 is oversized.

## 1. Vote offset code + should training be strictly balanced?

**Short answer: no, strict balance is not required — and don't discard data to get it.** Three reasons:

1. **Your data is already near-balanced.** The test confusion matrix shows ~10,002 negatives / ~9,946 positives. Post-dedup drift of a few percent is noise.
2. **The TM training gate self-regulates.** The (T−V)/2T rule damps feedback to whichever side is already winning votes, which absorbs moderate imbalance (down to roughly 70/30) without any intervention.
3. **The threshold sweep is your imbalance knob, and it's free.** That's exactly why `sweep_decision_thresholds` exists (§8): the decision boundary is a post-training integer, chosen from the full P/R curve, no retraining. Best-F1 at V > 22 is the sweep doing its job — some of that 22 is learned class asymmetry, some may be structural offset from vacuous clauses. The line below tells you which.

If you ever face *severe* imbalance (90/10): prefer, in order — threshold selection (free), duplication-based oversampling of the minority in the train split (no code change, deterministic), and only then per-class vote targets (a real engine change).)


**5. Open decisions.** (a) Framework-uniform `new`/`get` on enforced newtypes: keep as the documented deliberate naming exception (my recommendation — call sites are always type-qualified, and the edit protocol replaces whole type impls) or rename per-type — **your ruling**. (b) Dataset file (path, keys, positive label) — needed for the first real batch run, now in progress on your side; record its results against §8 criteria: (i) no macro-F1 regression vs. bag, (ii) obfuscation-suite win, (iii) production artifact profile.

**6. Known caveats carried forward.** Explainability offsets refer to PREPROCESSED bytes (raw-offset mapping = future per-stage offset tracking). Bag inference allocates one presence bitset per document (harness tier; reusable-buffer variant is backlog for production). Backlog otherwise unchanged: GraphTM-lite, caps-sentinel, packed-u8/bit-literals, Platt refill, stream evaluator, multi-class.

Report the rename-drop test results and your batch-run findings; the natural first item of Session 4 is reading those results through the S2-7 table.
...

---


## Recommended first diagnostic run

Against your current champion configuration on the deduplicated dataset:

```
cargo run --release -- --mode batch \
  --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto \
  --fire-guard 2000
```

This produces the unchanged 8-row baseline matrix (directly comparable to your take-4 results) plus four `+fireguard` byte-bag rows. The limit of 2000 means "fired on 2,000 consecutive shuffled documents (~2.5% of the train split) while holding a learned pattern" — deliberately assertive for a first diagnostic. If it proves too aggressive (reset counts in the thousands with degraded F1), the next sweep points are 8000 and 20000.

**How to read the results (three-way, S2-7 style):**

| Observation (guard row vs. baseline bag row, e.g. p0) | Interpretation |
|---|---|
| "always" count drops toward 0, F1 unchanged | Always-fire clauses were pure vote offset / dead capacity; the guard is a capacity-hygiene tool and best-F1 thresholds should move toward 0 |
| "always" count drops, F1 improves | Reclaimed capacity re-specialized usefully — the guard earns a place in the champion config |
| Reset total high but "always" count unchanged at eval | Clauses re-converge to the same ubiquitous patterns; the fix belongs on the vocabulary side (frequency-capped vocabulary — backlog candidate), not in more guarding |

---

## Handoff delta notes (append to the Session 4 record)

- **S4-1 (locked):** Fire-streak guard implemented for ByteBag: consecutive-fire streak over shuffled training order, vacuous clauses exempt, action = reset-to-fresh, zero RNG consumed, worker-count invariant, ephemeral (never persisted; kind-3 format unchanged; loaded models resume guard-off). OFF by default; CLI `--fire-guard N`, N ∈ {0} ∪ [16, 16777216].
- **S4-2 (locked):** Batch 2.0 — baseline matrix runs guard-off unconditionally (preserving comparability with all prior recorded batches); `+fireguard` byte-bag arms run only when the flag is active.
- **Error blocks now:** Cfg extends to 320; Bbg extends to 908.
- **Backlog additions:** conv-engine guard port via `ConvClauseWorkView` (vacuity = zero includes across all slots); frequency-capped vocabulary as the alternative remedy if the "re-convergence" outcome is observed; never-fire detection (the symmetric diagnostic — a clause with includes that fires on *zero* consecutive documents is starved rather than saturated; the same streak machinery inverts trivially, but that is a separate scoped decision, not this drop).
---

Hand-off:
```

# Handoff: Discontinuous-Set Ensembles ("Disco-Granmo") for Granmo

**Status at handoff.** Steps 1–3 of the plan (k-fold splitting, per-row prediction records, `row-audit` mode) are implemented and tested. This document records the design decided for the remaining work, the structural constraints of the architecture that the design depends on, the methodology that makes results trustworthy, and an ordered implementation plan. Nothing below has been implemented unless marked **DONE**.

---

## 1. Goal

Binary text classification with an **ensemble of high-precision, low-recall components**. Each component is allowed to abstain (predict negative) rather than guess; the ensemble predicts positive if any component predicts positive (OR-combination). The premise: a dataset's positive class is often a union of disjoint "regions," no single model covers all of them well, and separate models trained from different starting points (hyperparameters, engines, preprocessing presets, seeds) cover different regions.

Two delivery variants were decided:

- **MVP1 — rule pooling.** Extract clauses from several trained banks as portable rules, prune those that produce false positives, select a covering subset, and assemble them into one synthetic bank. Cheaper to run and evaluate; keeps a single-model artifact and prediction path.
- **MVP2 — model ensemble.** Keep separate trained models (possibly across engines and preprocessing presets), prune each of false positives by clause disabling, and OR-combine their predictions via a container artifact. Required when components differ in engine, vocabulary, or preprocessing, since such rules cannot share one bank.

MVP1 first. MVP2 only if the decorrelation test (§6.1) shows cross-engine or cross-preset components add coverage that same-engine rule pooling cannot.

---

## 2. Architecture facts the design depends on

These are properties of the existing engines. Any implementer must internalize them before touching the pruner.

### 2.1 Hierarchy
- **Automaton**: one `i16` state in `[1, 2N]`; include if `> N`.
- **Literal**: one boolean fact about the input, paired with one automaton. Conv: `(window offset k, byte b, positive|negated)`. Bag: `(shingle rank r, present|absent)`.
- **Clause**: the AND of its included literals. Fires iff all included literals are true. Zero includes → fires everywhere (vacuous). Conv clauses are evaluated per sliding window and OR-pooled: "pattern occurs anywhere in the document." The conv clause is a translation-invariant fuzzy n-gram template (wildcard slots, negated slots); it is *not* an absolute-position detector.
- **Polarity**: fixed by index parity. Even index → +1 for class 1; odd → −1. An odd clause is a positive pattern detector for class 0, not the negation of an even clause.
- **Vote**: `V = Σ fired(even) − Σ fired(odd)`; predict positive iff `V > θ`.

### 2.2 Clause behavior is fully determined by include bits
`clause_fires_at` / `bag_clause_fires` read only the masks derived from which automata are above N. Therefore a clause can be extracted as `(polarity, set of included literals)` and re-instantiated in another bank (included literals → `2N`, others → `1`) and it fires on exactly the same documents. **Rules are portable objects.** This is what makes MVP1 possible.

### 2.3 Clause indices are not comparable across runs
Slot 17 in two runs with different training seeds holds unrelated rules. All cross-run comparison must be by **rule content**, never by index. For the bag engine, additionally: the vocabulary is rebuilt per training split, so **rank** r is per-run; rules must be canonicalized with **shingle bytes** (via `ngram_at_rank`), and instantiated together only under a shared or union vocabulary.

### 2.4 Removing a clause is asymmetric (polarity-aware pruning)
Each clause contributes exactly one of `{+1, 0, −1}` per document. Disabling clause c changes V by exactly c's contribution, only on documents where c fired.

| Disable | Effect where fired | Can fix FP? | Can create FP? | Can cost TP? |
|---|---|---|---|---|
| even (+) | V −1 | yes | no | yes (accepted) |
| odd (−) | V +1 | no | **yes** (TN → FP) | no |

**Rule:** to eliminate false positives, disable **only even clauses**. To build high-precision negative components, disable only odd clauses. One direction per pass; mixing directions is non-monotone and "loop until zero FPs" is not guaranteed to terminate.

### 2.5 Removal must be disabling, not deletion
Physical deletion breaks: index-parity polarity (every later clause flips), the `ClauseCount`-is-even contract, storage geometry (`ta_states`, `allowed_masks`, `positive_include_counts`, bag mask/guard vectors), the `C+1`-entry `ProbabilityLut`, and artifact gate `ArtStateCountMismatch`. Mechanism: a **clause-active bitset** (`Vec<u64>`, one bit per clause, all set at construction). See §7.2 for touch points.

A disabled clause is a third category, distinct from vacuous (constant ±1 everywhere) and specialized (data-dependent). `summarize_vacuous_vote_offset` must take the active mask and report three buckets.

### 2.6 Conv engine limitations (recorded, not to be "fixed" now)
- One window per clause: cannot express co-occurrence of cues farther apart than K bytes; separated cues combine only through the linear vote sum. The bag clause can (`has "love" ∧ has "not "`).
- Stride > 1 fragments patterns across alignments.

Decision: keep the conv engine as the control. Develop pruning/pooling/ensemble against the bag engine first; maintain conv parity only where the `ClassifierEngine` surface requires it.

---

## 3. Methodology (non-negotiable)

### 3.1 Three data roles
- **train** — fit a component.
- **prune** — held out from train: clause drop-out, threshold selection, set-cover selection. Never used for fitting automata.
- **test** — held out from both: the only reported numbers.

Pruning on the training side is unrepresentative (FPs there are mostly mislabeled rows). Pruning on the test side contaminates every reported number. Expect prune-set FP ≈ 0 and test-set FP > 0; the gap is the honest measure of pruner overfit. Regularize with a **minimum TP-support** per surviving clause/rule (do not keep a clause firing on fewer than k prune-set TPs).

### 3.2 k-fold and the final holdout
- **DONE:** `split_dataset_kfold`, `--folds k`, `--split-seed` (independent of training `--seed`).
- Out-of-fold scoring (rules of model *i* scored on fold *i*) is sufficient for **per-rule statistics** (stability, always-fire/never-fire).
- Any object combining rules or votes from more than one fold model (synthetic bank, ensemble) has **no clean fold**. For those, cut a **final holdout** (~15%) via `split_dataset` *before* k-folding, run k-fold on the remainder, and evaluate the pooled object once on the holdout. Not yet implemented: `--holdout-percent` flag.

### 3.3 Variance reporting
One seed cannot distinguish 85.0% from 85.3%. All comparisons must report mean and range over ≥3 seeds or folds. `print_fold_summary` does this for k-fold; batch modes should adopt the same.

### 3.4 Data quality precedes everything
Mislabeled rows corrupt the pruner (it removes good clauses to satisfy bad labels), pooled-rule scoring, and any level-2 labels. Run the row audit (§4) and quarantine before ensemble work. Quarantine, never auto-delete: "never correct" is the union of mislabeled, ambiguous, and under-represented rows, and no statistic separates them.

---

## 4. Current tooling (DONE)

- `LabeledDocument::line_index` (1-based source line; stable row identity; not PII).
- `split_dataset_kfold(docs, k, rng)`; `KFOLD_MIN=2`, `KFOLD_MAX=50`; codes `DsFoldCountInvalid=713`, `DsFoldGeometryFault=714`.
- `--split-seed`, `--folds`, `--records-out`, `--score-train-side`, `--records-in`, `--audit-top`.
- `RowPredictionRecord`, `RowPredictionContext`, `append_row_prediction_records` (TSV `key=value`, appended, absolute path). Codes `Cli 811–814`.
- `HarnessRunConfig::score_train_side`; `ExperimentReport::{test_row_predictions, train_row_predictions}`.
- `--mode row-audit`: aggregates by line index; ranks by never-correct → wrong-even-in-train → strength of lean against label; prints review queue.
- `--model-out` rejected under `--folds > 1` (fail-fast, `CliInvalidValue`).

Recommended workflow before proceeding: accumulate records over several `--split-seed` values with `--score-train-side`, run `row-audit`, review the top of the queue, quarantine to `suspicious.jsonl`, regenerate the training file, re-run k-fold, measure the delta. Then proceed.

---

## 5. Features to implement, in order

### 5.1 Canonical rule export (prerequisite for MVP1 and cross-seed analysis)
**What.** For each clause of a trained engine, emit `(engine, polarity, sorted included-literal set, vocab_fingerprint)`.
- Conv literal: `(k, b, sign)`; comparable across runs with the same K.
- Bag literal: `(shingle bytes, sign)` — **not rank**. `vocab_fingerprint` = FNV-1a over `ngram_flat_bytes`.
**Where.** One function per engine beside `*_describe_clause` (the include-reading loops exist there); dispatch on `ClassifierEngine`. Output as a TSV/JSONL rule file, one rule per line, with run context (split seed, train seed, fold, preset).
**Also.** Optional near-duplicate clustering by Jaccard over literal sets, and per-rule **stability** = number of runs/folds in which the exact rule (or cluster) appears (stability selection, Meinshausen & Bühlmann 2010).

### 5.2 Cross-fold always-fire / never-fire analysis
Aggregate `EvalChunkPartial::clause_fire_counts` per canonical rule across folds. Three populations and their remedies:
- vacuous in every run → depth/epochs, not the guard;
- same *specialized* rule always-fires in most runs → vocabulary defect (stop-shingle, PAD run); evidence for a document-frequency ceiling / frequency capping in vocabulary build — **this analysis decides whether that feature is needed**;
- rule appears in one run only and never fires on its test side → seed-specific overfit; minimum-support constraint.
Note: the fire guard (`FireGuardStreakLimit`) is training-time pruning of always-firing clauses; the pruner below is post-training pruning of FP-carrying clauses. Complementary. If a recurring always-firer survives the guard ("resets nonzero, always-count unchanged"), the guard cannot fix it; only vocabulary changes can.

### 5.3 Clause-active bitset (prerequisite for pruning and synthetic banks)
See §7.2.

### 5.4 Greedy FP pruner (single model)
Inputs: a trained engine, the prune split, `engine_fired_clause_bits` per prune document (transpose to per-clause document bitsets).
```
FP = { d : label(d)=0, V(d) > θ }
loop while FP nonempty and not at target FP rate:
    for each ACTIVE EVEN clause c:
        fp_hits(c) = |{ d ∈ FP : c fired on d }|
        tp_hits(c) = |{ d : label=1, V(d) > θ, c fired on d, V(d) − 1 ≤ θ }|   # margin-critical TPs
        score(c)   = fp_hits(c) / (1 + λ · tp_hits(c))
    if max score == 0: break                        # remaining FPs unreachable by removal
    disable argmax(score); ties → lowest index
    V(d) -= 1 for every d where c fired; recompute FP
```
Deterministic (fixed tie-break, no RNG). Greedy weighted hitting set; trivial cost at this scale. Enforce minimum TP-support per surviving clause. Select θ jointly on the prune split via `sweep_decision_thresholds` after pruning (fresh-bank symmetry is gone once even clauses are disabled; θ = 0 is no longer a natural default).

**Baseline to beat:** the max-precision point of `sweep_decision_thresholds` on the unpruned model. Raising θ is the uniform version of the same operation; clause removal is worth having only if targeted removal beats it on test. Measure.

**Report:** precision/recall/F1 on test for (a) unpruned at θ₀, (b) unpruned at sweep max-precision θ, (c) pruned at its selected θ; number of clauses disabled; prune-set vs test-set FP counts.

### 5.5 MVP1 — rule pooling into a synthetic bank
1. Train k fold models (or N seed models; also across hyperparameter configs, same engine). Export canonical rules (5.1).
2. Deduplicate; compute stability.
3. Score every distinct rule on the **prune** data (out-of-fold for per-rule stats; final holdout reserved for the pooled bank): documents fired on, split TP/FP.
4. Exclude rules above an FP ceiling or below minimum TP-support. Greedy set cover over positive-label prune documents: pick the rule covering the most uncovered TPs; repeat; drop any rule whose TP set is a subset of the survivors' union. Deterministic; no random orderings needed (objective is monotone).
5. Assemble survivors into a synthetic bank. Bag: build a **union vocabulary** over the contributing models' shingles and remap shingle bytes → new ranks; conv: require matching K. Pad to an even `ClauseCount` with disabled clauses (5.3). Set included literals to `2N`, others to `1`.
6. Select θ on prune data; evaluate once on the final holdout against the best single model on the same holdout.

Persist as an ordinary kind-1/3 artifact plus the active mask (see §7.4).

### 5.6 Decorrelation test (zero new code; do before MVP2)
Save two models (e.g. the two IMDB configs; or byte-conv vs byte-bag; or preset `raw` vs `p2`). Score both on one test split with `--records-out`. Join by `line_index` and compute:
- **oracle accuracy** — correct if either is correct (ceiling for any two-member combination);
- **OR accuracy/precision/recall** at each model's chosen θ.
If oracle − best-single < 1–2 points, error sets are correlated and MVP2 is not worth its cost. Same-family models on the same data are usually correlated; decorrelation is expected from engine (positional vs frequency) and preprocessing preset (different byte streams), not from seeds alone. This decides the "fuzzy start" axes.

### 5.7 MVP2 — model ensemble with container artifact
- Prune each component via 5.4 to a **target FP rate**, not to zero (ensemble FP ≤ K·ε; requires ε ≪ 1/K).
- Greedy set cover over components' prune-set TP bitsets to drop redundant members.
- **Ensemble container artifact** (new kind): member artifact paths (absolute), per-member θ, per-member active mask, combination rule (OR initially), per-member preprocessing profile (components may differ — the "shadow administration" case; orchestration splits the raw input per member and rejoins votes).
- Prediction: run members (parallel is optional), OR the decisions. Report "which component fired" rather than a probability; the per-engine `ProbabilityLut` is meaningless for the ensemble. Recalibrate on prune data only if a probability is required.
- Evaluate once on the final holdout against the best single model.

### 5.8 Deferred / conditional
- **Level-2 veto TM over fired-clause bits.** A second bag-style engine taking the fired-clause bitset as `presence_bits` learns interaction rules (`fired(c2) ∧ fired(c4) ∧ ¬fired(c5) → FP`), which a single bank cannot express. Train with label = "level 1 produced an FP here," on data level 1 did not train on. **Gate:** first compute pairwise co-fire rates on prune-set FPs vs TPs from the transposed bitsets. If no pair co-fires on FPs materially above its TP rate, there is no interaction to learn; skip.
- **Group drop-out validation of the row audit.** Retrain once without the top-m audited rows; compare across folds. Improvement → detector finds noise; degradation → it removes hard-but-valid rows. Per-row leave-one-out retraining is intractable (`rows × seeds × train_time`) and is not to be attempted.
- **Coverage visualization.** Per component/rule: % of positive rows covered with no FP, overlap matrix (Jaccard over TP sets), % of positives uncovered by any component. All derivable from the prune-set fired bitsets and the records file; start as an external script over TSV.
- Conv port of the fire guard (already recorded backlog).

---

## 6. Implementation notes by code location

### 6.1 Section 1 — error codes
Add as needed, each with an `is_retryable` arm (match is exhaustive):
- `Cfg*`/`Bct*`/`Bbg*` for active-mask geometry (mask word count ≠ ⌈C/64⌉; padding bits set).
- `Ds*` for holdout-percent bounds.
- `Art*` for the ensemble container (kind, member count, path-not-absolute, member geometry mismatch).
- `Cli*` for rule-file I/O and malformed rule lines.

### 6.2 Sections 4–5 — clause-active bitset
Field `clause_active: Vec<u64>` on both engines, all bits set at construction, padding bits clear. Touch points:
- `conv_vote_sum` / `bag_vote_sum`: skip inactive clauses before the fire test.
- `conv_fired_clause_bits` / `bag_fired_clause_bits`: inactive → bit 0. `vote_from_fired_words` then yields the correct V automatically.
- Training pass 1 (`conv_scan_clause_range`, bag sequential pass 1): inactive clauses must not contribute to the V fed to `resolve_feedback_gates`.
- Training pass 2 (`bag_apply_feedback_to_view_range` and conv equivalent): inactive clauses receive no feedback; the fire guard must not recycle them. (Optional later: explicit "respawn" of a disabled slot to fresh state.)
- `summarize_vacuous_vote_offset`: accept the mask; three buckets.
- No change needed: `clause_fires_at`, `clause_fires_any`, mask maintenance, `validate_internal_consistency` (add a check that mask length matches C and padding bits are clear).
- `ProbabilityLut` remains index-valid (V still in `[−C/2, C/2]`); calibration drifts — recalibrate on prune data or ignore for ensembles.

### 6.3 Sections 9–11 — harness
- `--holdout-percent` (cut before k-fold via `split_dataset`, second seeded `FastRng` or same `split_rng` sequence — document which).
- `--mode rule-export`, `--mode prune` (load artifact, prune on a split, save artifact + mask), `--mode pool` (rule file(s) → synthetic bank), `--mode ensemble-eval`.
- Per-document fired-bitset dump for the prune split (input to pruner, co-fire analysis, coverage map).
- Batch reports: mean/range over seeds per row.

### 6.4 Section 12 — artifacts
- Active mask: prefer a **new artifact kind** (or a versioned extension) rather than mutating kinds 1/3, so existing artifacts remain loadable; alternatively store the mask only in the ensemble container and apply on load. Decide once; gates must reject mask/geometry mismatch.
- Ensemble container: absolute member paths, per-member θ, mask, profile, combination rule; version byte; fail-fast on any missing member.

---

## 7. Open questions for the next team
1. Mask storage: extend kinds 1/3 or container-only? (Container-only keeps single-model artifacts unchanged but means a pruned single model cannot be saved alone.)
2. Union-vocabulary remap for pooled bag rules: build from contributing models' `ngram_flat_bytes` only, or rebuild from the pooled training data? (Former is deterministic given the artifacts; recommended.)
3. Target FP rate for per-component pruning under MVP2: fixed ε, or derived from K and a global precision target?
4. Whether the vocabulary needs a document-frequency ceiling — decided by 5.2, not by design.

---

## 8. Success criteria
- **Pruner:** on test, pruned model's precision at its selected θ exceeds the unpruned model's max-precision sweep point at equal or higher recall, on ≥3 seeds/folds.
- **MVP1:** synthetic bank on the final holdout beats the best single fold model on F1 or on precision at matched recall, mean over ≥3 split seeds.
- **MVP2:** proceed only if 5.6 oracle gap ≥ 2 points; success = OR-ensemble on holdout beats best single member on F1 with ensemble FP rate ≤ K·ε target.
- Every result reported with mean and range; no single-seed claims.
```


...



TODO:
1. looking at first-test comparison
2. If always-fire clauses can be detected, can they be pruned? Can they be part of Gated Learning, or a v2-Gated learning?
3. probably focus on byte-bag
4. still work on team-diversity variation hopefully (part of batch 2.0?)
2. Design Batch 2.0
note: if it is possible to get meaningful difference from lighter-test setting, more tests can be done. deeper training tests can take hours to run.

For looking for better parameter settings:
(arguably a nuanced answer, e.g. as you can-should try a small change alone even if you expect one parameter to be tied to another)
What parameters are tied to others and should be changed with others, and which parameters can be raised or lowered alone?


TODO: RUn
```
cargo run --release -- --mode batch \
  --data /home/oops/datasets/json-cyberbullying-detection-dataset/Cyber_Bully_Data_binary_class_dedupe_v3.jsonl \
  --clauses 600 --vote-threshold 160 --stride 1 --patch 5 \
  --specificity 3.0 --states 200 --vocab-size 4000 --ngram-len 5 \
  --epochs 12 --seed 42 --workers auto \
  --fire-guard 2000
```


# Scope Planning:
- 1. Small fast do-one-thing-well models
- 2. Trying for a more performance model able to build on more data


# R&D:
- time series next byte prediction...

# Training Coach:
Is there a way to make a training coach or tools if not fully automated to progress test by test to find a good/best balance of parameters?

# "Stackable"
Is there a way to configure some kind of "stackable" learning, maybe like successive convolutions, that will 

# Continuous-retraining-learning types cases:

classic (if virtual) robot soccar-game using GM/TM?


# Genetic Alorithms and GM/TM? (related to team-diversity)
