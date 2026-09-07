Granmo Models Notes - Slim Production NLP


https://arxiv.org/abs/1804.01508
- original 2018 paper by Ole-Christoffer Granmo

https://arxiv.org/abs/2507.14874
- May 2026, Granmo et al
- "The Tsetlin Machine Goes Deep: Logical Learning and Reasoning With Graphs"

https://chemrxiv.org/doi/full/10.26434/chemrxiv-2026-dsz7v/v3 
Tsetlin Machine: A “Third Way” in QSAR Modelling, 30 April 2026 V3 Latest version
The Tsetlin Machine: A “Third Way” in QSAR Modelling
Authors: Mr. Paul Clarke  paulclarke61@gmail.com, Dr. Ivan Cmelo  , Dr. Runar Helin, Mr. Mayur Shende mayur.kishor.shende@uia.no, Prof. Ole-Christoffer Granmo, and Dr. Darren Fayn

Nov 18, 2024 VSAONLINE Webinar Series. Ole-Christoffer Granmo and Vojtech Halenka
https://www.youtube.com/watch?v=KngOz9e2saE 

https://arxiv.org/abs/2309.04801 
[Submitted on 9 Sep 2023 (v1), last revised 12 Sep 2023 (this version, v2)]
TMComposites: Plug-and-Play Collaboration Between Specialized Tsetlin Machines
Ole-Christoffer Granmo


https://arxiv.org/abs/2301.00709 
"To produce such logical embeddings, we introduce a Tsetlin Machine-based autoencoder that learns logical clauses self-supervised."
[Submitted on 2 Jan 2023]
Tsetlin Machine Embedding: Representing Words Using Logical Expressions
Bimal Bhattarai, Ole-Christoffer Granmo, Lei Jiao, Rohan Yadav, Jivitesh Sharma

https://arxiv.org/abs/2501.19018 
[Submitted on 31 Jan 2025 (v1), last revised 17 Oct 2025 (this version, v3)]
Scalable Multi-phase Word Embedding Using Conjunctive Propositional Clauses
Ahmed K. Kadhim, Lei Jiao, Rishad Shafik, Ole-Christoffer Granmo, Bimal Bhattarai


1 https://github.com/cair/TsetlinMachine 
2 https://docs.rs/tsetlin-rs/latest/tsetlin_rs/ 
3 https://github.com/RAprogramm/tsetlin-rs
4 https://github.com/topics/interpretable-ml?l=rust&o=desc 
6 https://crates.io/crates/zinfer-tsetlin

Resources for Multilingual Hate Speech Detection
https://en.wikipedia.org/wiki/Association_for_Computational_Linguistics 
https://aclanthology.org/2022.woah-1.12/
https://aclanthology.org/2022.woah-1.12.pdf

1. look at results of comparisons: 
- This Module-4 (byte-bag vs. convolution)
- older modules:
- flat-bow-tfidf vs. windowed
- cluster vs. flat

Overall clusters, windows, and convolution do not-as-well
but I think convolution might have other non-NLP uses.


"CTM" note: there is a convolutional-image machine in 
"Convolutional — Convolutional
2D patch extraction for image-like data." (https://github.com/RAprogramm/tsetlin-rs/tree/main)

- https://github.com/RAprogramm/tsetlin-rs 
- https://docs.rs/tsetlin-rs/latest/tsetlin_rs/ 
https://github.com/RAprogramm/tsetlin-rs/blob/main/src/convolutional.rs 

- clustering not helping? (e.g. old v1-v5)



multi-class: while multi-class may be possible, single-class may be optimal for a single-model,
e.g. try to engineer a large clean dataset with 'bad_hygiene' and 'good_hygiene' as the only two labels for prediction.

do-one-thing-well modules:
maybe spin off window modeling / byte-convolutionary modeling as a separate project,

if 'flat' works best, simply and focus on flat for one module (not high prority now)




2. Optimizations 1 parallel / concurrent
(the training does not use available cpu)
Strategies for Multi-Core TM Training:
Class-Parallel Training (Immediate 2x–6x speedup):
For multiclass TMs, clauses for Class A and Class B are completely independent during training. You can train classes in parallel using Rayon.
Lock-Free Asynchronous Parallel Training (Hogwild / TMU style):
Partition the training dataset across N worker threads[2][3]. Each thread runs train_step on shared or thread-local automaton states and synchronizes after every mini-epoch[2][3].
Note: Look at the tsetlin_rs crate on crates.io, which achieves 25x–90x speedups using bitwise SIMD clause packing (u64 bitboards) and lock-free parallel training[2][3][4].

note: rapid-demo-POC is usually not the same as production code. Unless there is a compelling reason, 3rd party crates should not be used.


3. Validation Gating:
A. for 're-training' 'continued-training'
B. as part of a training-workflow,
maybe: (using 10-row-sets because single rows might not be able to detectably improve performance...unless one row can improve performance (e.g. confusion-matrix FP/FN or derived metrics.)
1. train with a block of 10-rows at a time
2. if after that block, the performance fell,
systematically withhold 1/10 rows to see if 
A. performance improves
B. if performance falls less
If falls less, drop that and repeat,
loop,
keep training from rows that improve performance.

various goals;
1. validation-gating for updated-deployed systems to pass old tests
2. general training improvement to filter out counter-productive row-learning
3. 

standard in production ML and continuous learning systems (sometimes called Validation Gating, Regression Testing, or Checkpoint Rollback).
Because Tsetlin Machine state transitions are deterministic under a given evaluation set, this is straightforward to implement...

4 look at. other rust-crates for tsetlin machine models...
- convolution
- production-optimization
- performance: parameter-balancing etc
- thrifty bitwise operations

5. being able to use a dataset where the test-set is a separate file (or where you want to test it on a similar other set)
option for separate test-set file including
- test-set-file-path
- test-set-file-ylabel-col
- test-set-file-xdata-col

6. third mode:
1. train
2. predict
3. test-a-file

7. separate version: only predict-one binary, with a built-in model (for on-edge deployment, slim-end-points, devices, etc.), blazingly-slim pre-compiled model
- example goal: make one or more social-media-hygiene classifier (clean or not) model trained on a big compound dataset and test-set

8. separate version: only predict-one binary, with adjacent-file model

9. python-'refactor' library 

10. explainability reports
1. print-explanation-for-model (how classify: is json the explanation?)
2. print-explanation-for-input (why this one case classified? - features?)


11. explore thresholds: for models that score decently (above 90%?) see what 'parameters' are least-worst for that (e.g. 'maxy' is doubtless overkill)
e.g. 
english-toxic-language-dataset-for-nlp/toxic_comments_english.csv

93%
 --epochs 15 \
  --clauses 50 \
  --max-features 2500

vs
 --epochs 50 \
  --clauses 100 \
  --max-features 4000


12. data-set checker (maybe mostly done)
dataset_filechecker
TODO: remove newlines from text cols (option)
Probably a .py script using python-pandas (flexible csv reading etc)
A.
1. read in file
2. report bad rows
3. report .describe on columns, including mixes datatypes if found
4. report hash-compare identical rows
5. report very close rows (e.g. strip to alphanumeric, lower, levenshtein distance)
6. Q&A to configure cleaned-safe new .json out 

B.
- where there are separate test-train files: run a check (no changes made to files)
- check for row duplicates if not other tests

C. 
- takes in csv or json
- converts to csv/json


13.  Optimizations 2: Bitwise optimization for production model 'inference'


14: Model ~pruning, cleaning out low-"weights"?


15. Continuous learning production system...
- live-feedback learning?
(not MPV)

16. operate on bytes... maybe convolutional? (not BOW/tfidf)
- https://github.com/RAprogramm/tsetlin-rs/blob/main/src/convolutional.rs 
- 


17. Mode & Case Handling
- mainly for production-version 
A. prediction only (not train)
B. live-update continual training model


18. Granmo Byte Convolution for Time Series Classification



Note: The variation of performance in batch tests such as variations in improvement across different types of text preprocessing (e.g. where mismatched between conv and bag models) might help to illuminate use-cases for models and what model strengths and weaknesses are.
(and to highlight how data-preprocessing (perhaps like the 'tokenization layer') has a big role or impact on model performance yet exists 'outside of the "model"'). 


19. A continual stream evaluator, e.g.
byte-stream convolution modeling,
with some kind of paired-offset-window chunks to average-over start-stop points, e.g.
there are two 128 byte evaluators that return results after 128 byte, which might be in the middle of a word, so, in theory, comparing the output of these two offset by 64byte return schedules, it may be possible to do a continual stream reporting functionality with very light overhead. 
every 128 bytes
- bitwise optimized
- balanced between pre-processing-minimal and performance-best
- results likely sent to some Database or other endpoint for logging/alerting.

the main 'edge' case might be if a half-word somehow creates a false-positive... but that might be highly unlikely for the convolve-process. 



20. Learning On The Fly In the Ointment
Reinforcement Learning with Granmo Automata Game based Models/Architectures:
With a target being a social-story puzzle ( see https://github.com/stemnetbenchmarks/social_story_and_cookbook_puzzles  ), while it may be a 'difficult' task for an automata that cannot coordinate with another automata using a meaningful signal (so perhaps some pre-existing function-calls might be needed) given enough empirical learned-from experience, could Granmo Automata Game based Models/Architectures learn how to solve individual or classes of social story puzzles, puzzles that require sharing of puzzle state between participants, and ~coordination of actions to solve the puzzle?


21.
reporting on an examining what cases fail test: 
- in some cases more training variation needed
- in some cases they are invalid tests - bad data


22. Parameters: TeamCompositionPalette & Diversity (very speculative)

Properties of and variation in properties of automata:
- Question: Can, should, there be a new ~'model training parameter' (or more than one) for: board_species_team_diversity, to adjust how much variation there is properties of the agents in the game. 



23. Systematic comparison of results of different methods of text-preprocessing
- raw byte (no preprocessing)
- ascii lower
- N spaces, tabs, newlines -> one space
- 
Note: comparing on various texts and languages, not all languages are ascii oriented.

24. better explanation and understanding of different approaches/mechanisms:
- 'flat'
- 'convolusion'
- 'byte bag'
- 'windowed'
- 'cluster'
&
- "batch"


25. List of specific use-cases:
1. Hostile-etc language detection
A. For short-form docs such as social media
B. For any length (if chunked)
C. For continual streams

2. Text Extraction (maybe model pairs), item start-stop position in text for NER-type extraction, e.g. first-name, last-name, address-items, phone number, email address, age, etc. (e.g. 1. start position, 2. end position, 3. extract (maybe two models and one raw-code)

3. Various models for use in Modular-Eliza type models/architectures, e.g. a schedule question bot, do-one-thing-well focus, that translates unstructured incoming schedule questions into specific queries for a structured schedule database (mostly variations on: what is at time-T? What time is event-E?) Much of this can be entirely deterministic, but having resource-thrifty, explainable, Granmo-Models for various tricky-cases would be a boon. Possibly: language detection, past-future tense detection, requesting help detection, etc.

4. No-GPS location/direction updating: turn related, speed/velocity/distance related. Very resource and energy thrift for drone/automata

5. on edge deployment:
- mobile devices
- raspi or arduino 
- field-research devices

6. streaming audio signal compression (like vocoder)

7. video stream compression (single attribute detection for extremely minimal representation)


26. Misprediction Inspection:
Analyze misprediction_log.txt generated during training, to see if errors are from data corruption or label noise vs. from model blind-spots


27. Test-Report Suite 1.0

I think it makes sense that some overall next-steps should be improving the test-suite, e.g.

1. clear and detailed logging (missed-cases logging (maybe implemented now), and test-results), display, and results report printing (e.g. model-comparison reports)

2. for comparing models, as in batch should models be compared in real time, or can a N-logs from N-model-types be compared after running for analysis? (perhaps, json of log-data for a given test-set)
note: test train split being random, maybe requires seed?

3. while some may be presumed dead-ends, I want to formally compare:
1. clustering (or cluster + flat-bag)
2. byte-bag
3. bow-tf-idf (previous code can be shared to merge)
4. convolution (I suspect this may be good for something...if not for ascii text)

4. Trying to get a few different quality-check-able (ideally) types of tasks/datasets
1. language Hygiene (current domaine)
2. "NER" (id/extraction by type)
3. time series prediction
4. embedding-vector version of Granmo model
5. maybe logistic type probability estimation
6. ...
7. numerical input https://www.kaggle.com/datasets/jeyasrisenthil/input-data 

5. 'features' to add for more testing
- "conv-guard"



28. Explainability Framework

29. Ongoing Performance Benchmarking Framework
A. Static Golden Set
B. Assorted Test-Sets (e.g. added over time)
C. Past-Times Benchmarks (past performance vs. current)
D. Recent-Data Benchmark (Recent-cases (not in past test/training sets))


30. MER integration
- SchedulELIZA

31. Uma Integration
- A. Language Hygiene


32. Use-Cases and Datasets
- https://www.kaggle.com/datasets/haideradnan77/mental-health-condition-classification 
- https://www.kaggle.com/datasets/ahmadrosyidalfualdi/student-mental-health-dataset 
- https://www.kaggle.com/datasets/sohamchaudhari2004/sentiment-dataset
- 


- Time series and next-byte prediction (generalized?)
- Swarm Learning: Maybe levels of similar problem-space: swarms of interactive TM/GM machines, perhaps coordinating around different 'places' in the overall problem space

Crazy mnist test; tried running byte-conv on mnist converted to single digit identification, better than baseline (and not only predicting one class) but not great.


Quasi-Probability:
- maybe a simpler system: instead of having an overall vote-threshold for prediction (of a binary class),
you report the raw 'votes' (perhaps divided by the total for a normalized scale), to get a 'degree' rather than binary outcome.
For topics like 'language toxicity' there are use-cases for 'how much' or 'to what degree' rather than 'yes or no'
(and possibly another level, maybe swarm-ish) where the various output of various separate models may be used to train another model on another task.
- 


Q: What kind of 'explainability' report (maybe like feature importance, or not?) can be produced after/during training?


32. Separating Feature-Creation Stage vs. Feature Importance Stage:

A strength of deep learning is the flexibility of feature engineering being added to the statistically-based feature weighting. Arguably this is at least part of Sutton's (arguably nuanced) 'Bitter Lesson' paper, that using better process and more data is better that relying on human-experts to hand-design the features, just as earlier it was accepted that even human-selected features are best weighed using statistics rather than human intuition. 

Though it may differ significantly by use-case and by modeling-type (convolution, byte-bag, classification-head (single class, multi-class), regression, logistic-probability, next-byte prediction, time series, NLP, audio, video, IoT data, finance, Fin-Tech, Health, etc. etc.):

I am still just starting to learn about Granmo Models (TM etc.) so my understanding is most likely not entirely correct.

Is it the case that at least the currently-tried:
- Clustering + BOW-TFIDF
- Windowed BOW-TFIDF
- Flat BOW-TFIDF
- Convolution-Byte
- Byte-Bag

Models are using raw N-Grams as the only source of features, then values for those get decided in distributed game (which perhaps could be extrapolated to on-Edge swarm and multi-sensor distributed model training, maybe).

(Instead of having a pre-set N-Gram (raw, stem, preprocessed, etc.) feature-space:)
While it may be more detailed and entailed, could we look at the byte level at byte classification or byte prediction (possibly only incidentally different) as being highly non-linear wave-functions within a fixed 8bit-byte 0-256 value range. 

Derived-Features:
Various kinds of approaches (such as Latent Dirichlet allocation (developed for DNA sequences before NLP use), and various Deep learning approaches from convolution to RNN to LSTM to Attention to Diffusion, etc.) create a feature-space, with the Firth-concept-space being especially famously effective (if also famously expensive). At the same time the lack of paper trail about what these derived features are may be part of the somewhat semantic disagreement about how 'explainable' deep learning models are: the feature-formation process and 'feature log' is (I think) rather buried in the overall soup of the deep-learning pool of 'weights' (again: the theme of separating feature-derivation from feature-application). 

To recap:
It seems (I may be entire wrong) that the models up until now in these experiments, if not with Granmo models more broadly since 2018, have strongly or exclusively focused on the feature-use/feature-application step/stage, without there being a similarly voted upon discover-derivation process whereby either features as linear and nonlinear functions or features as a separate encoded concept-space.

Also, a perhaps hidden area of this is another feature-deriving step of pre-processing data that for much NLP is separated into a tokenization step which arguably has a very significant effect on up-stream modeling. And strangely: where is the bitter lesson here? We spend billions of dollars on trillion/quintillion weight models and yet we don't trust those models to do a good job of deciding how the tokenization proprocessing is most optimally done? (In saying: "We smart! We know what those first tokens should be!" have we not failed to learn the 'bitter lesson'? And how is this chocolate-bitter lesson pejorative?)

~Heavy-Features vs. ~Functional Features: (Apologies for the terminology)
An area to consider is how to approach feature derivation. While it might be possible to mirror transformer models with a GM/TM backend to create an equally expensive and large model with a other perks: bitwise not floats, better audits, better pruning, better continued-training, etc,) that is not the direction intended here. 

I am primarily interested in the light-weight, portable, hardware-friendly, Edge-friendly, IoT friendly, low-power-compute for prediction (if not also training) friendly, 'slim and lite' properties of Granmo models. While transformers (and maybe other types such as samba and diffusion) have demonstrated the power of Firth-concept-space, the enormous cost has also been demonstrated.

Are there other ways of getting some more nuanced features and pre-processing steps to feed into the 'end-stage' modeling of weights that get voted on? And is there 

Note: While firth-concept spaces for general models are absurdly large, do-one-thing-well models only need arguably minimal (if articulate) manifolds within those 'general' spaces (most of which is useless for a 'hot dog, not hot dog' model). 

Is it possible to go some depth (and perhaps deep in a thrifty way), given enough data, looking at, perhaps in a Firth approach, not just how each isolated raw input 'part' (n-gram) relates to the prediction problem space, but how relationships between (concepts across) individual raw data points then relate to the end-stage problem space. 

One example or analogy for this may be MNIST, where a 'concept space' of edges and curves and other things is the key to associating pixels to the end-stage problem. As hoped before, for MNIST this does not require the entire universe of all general concepts, just enough for the one task.

So how are MNIST and NLP different? So far as I know, no one claims that "image tokenization" using a one-size-fits-all template (as in the case of n-grams, BOW, and other static tokenizers)) is needed for MNIST and similar tasks. Why not? If image-concepts can be derived by comparing data rows (including 'vertical' edges in serialized data, more alien than audio-time series, text, or DNA-code) how can't at least some word-concept-relationship. This is not to say that any model-archiecture will automatically handle all feature-derivation in every possible way: convolution is one architecture-defined way, RNN another, LSTM another, Attention(transformer) another, etc.) 

(Note: As is traced in the 2026 book "On the Mark: A History of Punctuation from Ancient Egypt to the Emoticon" by Florence Hazrat, the human-concept of having language be space-delimited standard-spelling letter vs. number vs. symbol notated is historically very recent, with most classical language being both non-delimited and where spellings (assuming a given language has the concept of a word at all, e.g. Asian languages) were continually morphing (more like 'Finnegans Wake' than a webster's dictionary). That 'tokenization' helps digital-machines may not be unrelated to how it also probably helps human-biological-machines. But 'tokenizing the stream', like finding edges in an image-file, is part of the modeling process, not literally alien to it (though human-reading has been somewhat pre-digested by preprocessing on a tokenized page).) 

By design or for economy, should we try to cast a wide or narrow net for how feature-derivation may be handled within the modeling process? As different deep-learning architectures (may, if my understanding is not too mangled) orient towards a specific approach to feature-derivation (CNN(narrower), RNN, LSTM, Attention(broader), etc.)? Will there need to be specific GM/TM architectures that are oriented towards a type and breadth of manifolds? And is there some or any direct correlation between GM/TM feature-derivation methods vs. Deep-ANN feature derivation-oriented architectures and techniques? Maybe...maybe not.

Am I suggesting effectively adding 'hidden layers' to a Granmo-Model? May not necessarily: as mentioned above, making a deep network with a Granmo backend could be effective, but that is my goal here. For example, there already are convolutional-function GM/TM models in standard code repos 
( e.g. https://github.com/RAprogramm/tsetlin-rs ), and those do not require deep-learning in order to convolve a function along a sequence. 

This might vary depending on what the use-case and GM/TM/architecture is (as suggested above). Serialized data such as image data may be different to work with compared with native-series time-series data (audio, prices) vs. NLP natural language and perhaps DNA that have other properties (e.g. word-like abstractions). 

Generation and prediction: Could it be a clue that unlike unsupervised generative model training, for do-one-thing-well labeled data, that context may be material for constructing (confabulating) a raw -> concept -> pattern map. If we have (at least) two classes of data, we already know what the relationship-pools are. How do 0-class-label series patterns relate to each-other in ways that 1-class-label series patterns do not? THis could be passing/convolving functions along the sequence, or chunk-bagging, or looking for byte-wave-functions (or functions of functions), and ideally using the entire sequence as a sequence may be useful when possible (not BOW) (how a string starts and stops may be as relevant as the middle).

Even if the functions-on-functions are elaborate, could we not describe the pattern-matching done by a BOW spam filter as a series of byte-stream wave-functions? Preprocessing is a function. Stemming is a function, tokenization is a function. Relative frequency is a function. Associating 'company they keep' words into concepts is a function. 

Specific project/contexts in progress include:
- user sentiment detection (text)
- bullying/hostility detection (text, using kaggle data currently)

- NER-type data extraction for standard fields: name, phone, age, address-fields, email, 
- specific question identification: 'asking about the time of a known event' & 'asking about the event at and known time' (and extraction: what event, what time) (probably a 'place' dimension of the same)

- image/video: facial emotion/micro-expression classification/detection
https://www.sei.cmu.edu/blog/revealing-true-emotions-through-micro-expressions-a-machine-learning-approach/ 
- audio: voice emotion/micro-expression classification/detection

- audio encoding/decoding (slim vocoder) (lossy, but audible)
- very slim video encoding/decoding (lossy) (currently undefined)

- no-GPS drone video/sensor direction(change) & speed(change) = location(change) [no data for this yet]

- IoT health-sensor data: [no data for this yet]

Types of Systems:
1. Production Inference/Prediction:
2. Production Continuous Learning (or period extra-training)


possible others:
- (colour/pattern of one specific object in image)
- rat-maze NPC/MOB behavior models



Note: currently a byte-bag is working better for NLP classification that byte-convolution (not surprisingly)
I am guessing that image vs. audio vs. time-series vs. language (and specific projects in each) will favor a specific approach (not a one-size fits all)



https://arxiv.org/abs/2309.04801 
[Submitted on 9 Sep 2023 (v1), last revised 12 Sep 2023 (this version, v2)]
TMComposites: Plug-and-Play Collaboration Between Specialized Tsetlin Machines
Ole-Christoffer Granmo


https://arxiv.org/abs/2301.00709 
"To produce such logical embeddings, we introduce a Tsetlin Machine-based autoencoder that learns logical clauses self-supervised."
[Submitted on 2 Jan 2023]
Tsetlin Machine Embedding: Representing Words Using Logical Expressions
Bimal Bhattarai, Ole-Christoffer Granmo, Lei Jiao, Rohan Yadav, Jivitesh Sharma

https://arxiv.org/abs/2501.19018 
[Submitted on 31 Jan 2025 (v1), last revised 17 Oct 2025 (this version, v3)]
Scalable Multi-phase Word Embedding Using Conjunctive Propositional Clauses
Ahmed K. Kadhim, Lei Jiao, Rishad Shafik, Ole-Christoffer Granmo, Bimal Bhattarai



26. Tradeoff Separation: No One Pattern (Geometry) is All Patterns, So Separate the needed Patterns.

E.g. Parallel Features

Step 1. Try a variety of different feature approaches.
Step 2. Run separate (Ensemble, perhaps) models based on most effective feature sets.
Step 3. Maybe tricky, maybe do-able in some cases: Combine N clearer signals into a compromise prediction/inference, rather than one muddled signal.

NLP may be an area with some clear cases of this.
When pre-processing text, raw, porter-stemmer, lower, strip, remove symbols (or convert specific symbols e.g. leet), etc.

A perhaps crude question is why not try both/all?

Another part of this question may be how to avoid interference between alternate features?


A. Semi combinable variations:
- Porter + raw
(maybe issue of double-signals)

B. Significantly Different Types:
e.g. presence(count/frequency) vs. pattern
- Weighted String Matching
- TFIDF

- Sequence-pattern of letter 'e' or string 'not' 


1. Parallel Encoding/Preprocessing

There are various ways to 'standardize' (or not standardize) a text:
- lower
- stemming/lematizing
- stripping duplicates (space, tab, newline)
- merging (space, tab, newline)
- symbol removal

Known: Depending on the task-context (case-by-case), more signal or noise may be captured or lost with a given type of preprocessing. E.g. Sometimes symbols (not letters or numbers) or use of capitalization are key to the signal, sometimes they are noise that separates 'n-grams' that should be merged.

Question: To what degree is this also the case within tasks? To what degree are tasks themselves ensembles of different types of patterns, e.g. for some n-grams raw is best, for some n-grams lower is best, for some n-grams stemming/lemmatizing is best. This might be a case of having a dedicated 'feature-derivation/selection' process (rather than assuming a static feature set or a one-sized-fits-all process).

There may also be [difficulty] levels for this. e.g.
A. for a whole task, version-N of an n-gram is best 
vs.
B. where it is not 'better-worse' but different, and ideally both are used. (perhaps such as where 'color' vs. 'colour' in travel-doc NLP being merged or not will have sub-task pros and cons) (Or is this a case where both can be used?)

What are cases where both pre-processing types cannot be simply used in the same overall pool? Can we assume that in most-cases most n-grams would NOT be *N represented such that simple-pooling would balance out the duplicate signals because everything was evenly duplicated? Are there cases where 'simple pooling' may be a cheap and quick method of integration? What are some of the factors around this? This may be another simple yet not-simple NLP question where various factors case by case. This is likely overly-specific to NLP, especially non-sequence analysis.

For cases such as sequence analysis, there are probably few cases where any form of input duplication (multiply the inputs (create new whole versions of each) and run them all) would be useful; though it should be simple to test that at maybe good to double check.




If there is a way to re-integrate the signals, could an ensemble approach boost performance?



which may or may not blend into ways of looking at chunks of text: 
- short n-grams
- long n-grams
- tokenizing


2. Count-Frequency vs. Pattern-Function

Perhaps for characters and n-grams, is there a way to standardize basic pattern-functions into a feature-set that could be a proxy for count-based TF-IDF


Q: Is there such a thing as discontinuous N-grams, e.g.
"best * ever"

E.g. Regex-Grams?


Frequency Comparison:
It may be useless but I am curious, what would be different ways to look at the time-distribution-pattern of, e.g. the letter 'e' when comparing two types of documents? ('!' Exclamation points in hate-speech may be a more obvious example, but 'e-classification' seems like a good R&D sandbox for various NLP classification tasks.

Maybe vowel-distribution-functions are not useful in most cases, but if they are cheap and they are useful, why not have them in the tool box?

https://www.kaggle.com/datasets/amananandrai/ag-news-classification-dataset
https://www.kaggle.com/datasets/abdallahwagih/spam-emails 
https://www.kaggle.com/datasets/tblock/10kgnad 


# Production NLP
Something to keep in mind overall is the real-world cases for multi-lingual NLP applications, where a process hyper-optimized for ascii-English may be academically best-in-class, but not practical for real world use.




Meta-data analysis:
- proportion of symbols to letters
- proportion of capitalization


27. Count vs. Distribution Pattern in NLP

Broadly speaking, in simplified overview, my understanding of 'best-performing-models' in NLP is that: 
- for big models and big-data the Transformer-Attention deep learning approach is the breakout leader
- for small models, small devices, and small data, and narrow models, the non-sequence statistical learning BOW & TFIDF approaches can be adequate.

There are two branches here:
1. sequence vs. non-sequence (BOW)
2. symbolic vs. sub-symbolic

The sub-symbolic, I would at least tentatively assert, is likely using both sequence and non-sequence elements (directly or indirectly) (perhaps moot, or incorrect).

The area I want to ask about here is symbolic sequence analysis, in particular using Granmo(T-Machine) approaches.

There are probably non-deep learning ways of looking at 'sequence features' (or not?) that have been parts of not-NLP data science (during the BOW NLP era). 

Even if not likely to be 'best in class,' are there any sequence features that can be defined contrasting labeled classes for NLP/Genomics?
(E.g. I think LDiA starts with Bag-of-Ngrams, non-sequencial). 


Feature-izing distributions (maybe):
Let's look at the 'letter e' example:
Between two labeled classes, how could we look at sequential distributions of the letter e between the two classes.
Can we identify re-find-able properties that tend to be found in one class rather than the other?
28. Tradeoff-Detection
Can it be detected when a model is being pulled N directions:
1. If so, instead of 1-compromise model, make N  specific models
2. Try to arrange it so that false-positives are minimized, and any positive from the suite will count as positive.





29. Disco-Granmo: Discontinuous-Set Ensembles / Sub-Problem-Partitian Ensemble
"No one-pattern is all-patterns, and rarely is one empirical pattern exactly one theoretical pattern."

(Note: making a better cross-random-seed comparison architecture may be a pre-requisite for Disco fun.)

For binary classification:

Training Process:
1. Fuzzy Start: 
Using N seeds (maybe 2 or 3 will do to start) as separate workspaces, in each workspace: creates (fuzz) several more or less randomly parametered models.

With or without a process to make one or more models less fuzzy. That each model may only correctly predict a minority 'region' of correct classifications is a feature, not a bug.

(sequence of 2-first or 3-first (which is 2 and which is 3) may not matter)
2. Remove False Positives (False-negatives are AOK) (some narrowing of scope (loss of TP-predictions is more a feature than a bug).

3. Compare models across seeds and look at overlap and select a set of component models that cover as much of the dataset as possible; possible method: leasts-worst-subtraction. Remove any models that are entirely overlapping with other models (maybe comparing random subtraction sequences).

4. Some way to call each model in ensemble (maybe in parallel), or the equivalent.

Note: this might employ more or less different models that are seeing different patterns, such as sequence-based (convolution) vs. frequency based (bow/tfidf/byte-bag), and using different preprocessing (raw, lower, strip, stem, remove-stop-words, symbol-substitutions, etc.)

Note: The main part which I have not figured out is feasibility or how to remove false-positives. My first two thoughts/speculations are: 
1. That the internal process of voting/criterial could be tweaked for an 'imbalanced' rather than balanced outcome.
2. 'Drop-out' test where (across random seeds), if disabling a clause (or ~clause-set) removes a false positive: then disable the ~clause-set, repeat until no more false positives remain. (I am still looking into how clauses are interconnected (e.g. even numbered pairs and sets of connected pairs?)

There may be ways already in the dencentralized game that are more elegant and lead to a similar outcome.

This may not work, but part of the thinking comes from how significantly different starting parameters can be later refined to two different equilibria:

For IMDB review dataset:
```
  --clauses 300 --vote-threshold 75 --states 85 \
  --specificity 3.7 --vocab-size 8000 --ngram-len 5 \

```
vs.
```
  --clauses 400 --vote-threshold 105 --states 85 \
  --specificity 4.2 --vocab-size 10000 --ngram-len 6 \
```
Both perform around 85% (but I suspect not exactly the same 85%, and just maybe two heads are better than one).
#### Species Diversity vs. Ecosystem Diversity (analogy-terms)
One issue or factor in disco-style ensembles, is how similar the models are. If models are 'species variants', they may be able to be (if desirable overall) combined in a more internal way (perhaps within the already 'decentralized' voting system. But as may have been shown in the case of early experiments with combining a frequency-based model and a sequence-based model into a crude ensemble, the outcomes is worse performance not better, such that the two are interfering and not supporting each-other. The 'no-false-positives' policy may be part that problem space. But another type of factor (again going back to pre-processing sometimes being a kind of 'shadow-administration' second-model) is where there is a different preclusive pre-processing of the data, requiring another level to orchestrating the ~parallel ensemble 'breakdown-buildup' leading to a single result from a single input (after being split and then (re)joined in the middle).

Another design question note is whether it is better to add this Disco-Ensemble functionality to the current code base (and make a disco-batch comparison (or more than one) testing feature) or whether it conflicts and should be a repo-fork. Being able to add to a larger set of compare-able tools would be ideal, so that during data-set-exploration tests can be run to show the relative-effectiveness of different approaches (as in the batch-test feature). 


#### Representing & Visualizing 'Populations' of Patterns: To understand the problem-space and the dataset
As with the classic and eternal 'Genie in the Lamp' problem in Data Science, without future information or insights we do not know (or cannot know) at any given time if the question we are trying to formulate is malformed, such as trying to incorrectly combine multiple questions into one or inadvertently asking more and more ambiguous questions that we intended, or asking a technically-too-narrow question (such as the robot that learned to 'toss up and catch a ball without dropping it' by micro-vibrating the robotic hand).

A first crude form of visualizing Discontinuous problem spaces (perhaps especially common in vaguely defined natural-language topics and datasets) may be a %-coverage map of how many and what size 'continents' of the dataset (across random-seeds for splits) are covered by no-false-positive models, and what % is not covered by any model. If possible it would be useful to find overlap/tradeoff relationships, or the 'distance' between what 'data-rows' are predicted by each model (how much overlap). 


Note on comparing clauses: Because clauses may be listed in arbitrary order (the index does not tell you which clause is which)
If outputs (e.g. clause_fire_counts & clause_include_totals) slot number indexed, then to do cross-seed analysis the harness should have the clause rule in addition to the index. A byte-bag engine rule could be written to use shingle bytes (not vocabulary rank).


Idea/Question: As there may be situations where a clause sometimes produces 1 or 2 (out of 10k) false positives, but boosts f1/accuracy significantly: maybe go to have user select how many false positives are acceptable, e.g. u8 0-255


30. 'Fire-Guard,' Bad-Data-Row Screening, & Seed/Cross-validation Questions:
Q1: Should the fire-analysis and fire-guard be (if possible) analyzed across seeds? (part of larger question about if k-fold cross-validation can be implemented)

Q2: Should some kind of data-quality or row-suspiciousness examination be done before penalizing clauses/rules for how they behave on an erroneous data-row?

Question/Idea: Since ~clause rules are clearly interpretable, would it be possible to do a kind of (maybe cross-validation-ish) iteration/comparison across N different-random-seed-split train-test sets to identify:
A. ~clause-rules that either always fire or never fire with any train-test split configuration
B. to combine/pool ~clause-rule-sets that cover all positive-ID cases across N random-seed-split comparisons.

(and logging which clause-rules were most subject to occurring in rare splits might be useful (vs. common rules that occur in every split).


31. Identifying rows that are never correctly predicted across any random-seed-test-train-split
- e.g. possibly bad data (very common in large NLP datasets)


32. A Clause-Set Level
Using this example:
```
c0(+) c1(−) c2(+) c3(−) c4(+) c5(−)   V   label  cell
doc A         x                 x     x           +1    1     TP
doc B         x     x     x                       +1    0     FP   ← target
doc C               x     x           x     x      0    0     TN
doc D         x           x     x                 +1    1     TP
```

Might it be possible to try conditional rules, where cN clauses need to have other clauses be true or false in order to active? If clause c2 & c4, then c5 must also be observed.

Q: Could a 2nd Granmo-TM model be used as a multi-class classifier to see what clauses are dependent on other clauses to cause/prevent FP (or false-negative? (E.g. using data from across many-random-seed tests...maybe)

33. Data-Examination (MPV-1 Done)

Note: It seems like a 'data-examination' step may be best practice, whereby this or another process repeatedly FN or FP predicted rows are double checked to make sure they are valid data.

On the other hand... there are various ways of possibly screening for outlier rows (with no possible deterministic way to divine mislabeled data): 
A. Look at which rows are excluded from a conservative positive-prediction-only model (or disco-ensemble)

B. Maybe: Compare across different random seeds how a row behaves both in the training set and in the test-set. Does inclusion in the training set lower the performance of the model: maybe: False-Positive/False-Negative Drop-Out: trying with a few random seeds, if dropping a row has a positive effect (across seeds) then move that row to a 'suspicious' file. 
1. This could take a lot of time to do, but the process might still be effective.
2. If they can work using quick-shallow tests, it might only take a few seconds to iterate through tests.


34. K-fold Cross-Validation should allow for (easier) clause-pooling


35. Structured-Extraction / Structuring-Extraction

Use-case: Finding/extracting the event-name a string (in a time-question).

Common question forms include:
- When is {even_name}?
- What time is {even_name}?
- {even_name} starts at what time?

Because spaces between 'words' are small-quantity-quantized, one strategy may be to compare the various space-delimited-chunks of the input sentence each into a 'front-section-detection' model. E.g.

Note: there will be cases where the target section is at the start or the end of the string.
Examples:
1. "{Movie} is when??"

2. "When's the {movie}?"

3. (Example taken from the synthetic training set)
["At what time does the system maintenance window begin?"]



Starting with example 3:

Since the boundary will be a space, 
'Space' delimited inspection-Options are:
1. [ STRING_START and // + starting edge-case
2. ["At]
3. ["At what]
4. ["At what time] 
5. ["At what time does]  
6. ["At what time does the]  
7. ["At what time does the system]  
8. ["At what time does the system maintenance]  
9. ["At what time does the system maintenance window] 
10. ["At what time does the system maintenance window begin STRING_END] // + end edge-case

(Where either option 4 or option 5 will work)

### Speculation 1: Playing Heads and Tails
Two models, one to detect 'before-the-inside', and another to detect 'after the side' 
Various sized chunks (either all, or a window) from the start of the string are fed into the model and each receives a classification (or a set of votes).

In this case, a spike in no-votes would be what we are looking for.

(these are votes, not percentage/probabilities):
By looking at the overall yes-vote profile, it should look like this (would the no-vote be the opposite?):
1. 90
2. 90
3. 85
4. 85
5. 90
6. 5
7. 0
8. 0
9. 0
10. 0

(note: there is an edge case for adding a check-spot at the end, but should be possible)

And a similar process may work at the end.
This is not an end-to-end byte-index output, but it may be just as good for short sentences and a small fast model (and this few could be run in parallel on most systems). 

One idea for that might be a 'bi-directional' approach, where the second tail-detection is literally the same process but run in the reverse direction-from the end. 



### Speculation 2:
Another idea is to map standardized 'spaces' to the (bytes of the) words/tokens of the preprocessed-cleaned string (tab, newline, multispace = one space, including the start and the end of the string) 


## Single Classification: Inverse of Not-Known-Outside (instead of B-I-O)

Instead of trying to affirmatively define Beginning, Inside, and Outside, (or the boundaries of those (meaning 5+ definitions), let's explore the STEM-elegance of negative-definitions:

What we want is an unknown (and at times forever novel) 'inside', in a context where there is no 'classes' or states matter: We do not care about before, after, begin, starts, stops, boundary, middle, etc.). We are looking for something that is NOT what we know well: the well known 'outside.' We can (try to) classify a very stable and well known 'is-outside' state, and use the inverse of that to show where that is not the case: Inside = Not-Outside. We build a model of the known outside in order to see where that model predicts 0-false (or has a low vote of confidence). 

## 'Outside' detection for "not the known outside" Binary-Classification:

We can set up training data designed to create a dataset with 'Inside-or-Not" classification of each space (with space/tab/newline standardized to 'one space' in preprocessing) (plus a position or space at the start and end): where 'outside' is 'exclusive' of the spaces before and after the target. (e.g. the goal is to identify a window slice that is divided at the target-space as a boundary, as a divided spaces, as a not-outside space. To do this the boundary spaces themselves are defined as 'inside'. e.g. It will happen often that one single word is either the inside or outside portion, so the requirement cannot be to find an boundary space inside and inside that has no spaces -- even if that were possible it would punt the task of identifying where the boundaries are (not where inside points are). The goal is to define boundaries using a 'not-outside' vs. 'is-outside' classification.).

Running this model in one pass iteratively over the input sentence (once for each "space" (+ start + end)) may be the only definition we need to seek the start and end bytes around the 'inside.'

Thought on modeling inside or outside: In real life (new real life data will often not be in the training set), the 'outside' text (not-inside) is what is most predictable. The inside will contain ever-novel content. So if possible "not known-outside" would be a better known-target. 

As an example of this inverse 'known-outside' modeling (where 'outside' is a very stable predictable classification) using the above sentence:

Votes for 'Is Outside' classification:
1. 90
2. 90
3. 90
4. 90
5. 85
6. 5
7. 3
8. 1
9. 2
10. 90 // edge case added end-check

In this case we look for lower-votes to see where the inside is.
(re-note: there is an edge case for adding a check-spot at the start and end, but should be possible)

However: How does one ask a TM model about a particular space or point? Are the options
A. N-bytes/chunks before the space.
B. N-Bytes/Chunks on both sides (what about start and end positions? ... we could literally append a known nonsense string to the start and end of each string as part of pre-processing or use STRING_START and STRING_END so that there is always something on both sides)
C. N-Bytes/chunks after the space. 

But doesn't that narrow the model to looking at extremely short sections of text? I might come down to a balance of byte window-size.

Returning to examples:
1. " STRING_START {Movie} [is] when?? STRING_END "

2. " STRING_START When's [the] {movie}? STRING_END "

3. (Example taken from the synthetic training set)
[" STRING_START At what time does the system maintenance window begin? STRING_END "]

Perhaps the 'space' inspection idea means to iteratively (from each 'space') grab (in short: the word to either side of that space.

Window slide example:
1st. STRING_START {Movie}
2nd. {Movie} [is]
3rd. [is] when?? 
4th. when?? STRING_END

This would get four sets of yes-votes and no-votes:
yes-votes for 'is outside' (not probabilities):
1. 0
2. 0
3. 90
4. 90

no-votes (not probabilities):
1. 90
2. 60
3. 0
4. 0

Wherever a boundary-slice is not classified as 'outside' that effectively means that that space is a boundary space.

Edges cases and vote vs. classification:
1. The easiest approach may be to take the classification the model gives, not the vote. But for planning looking at the vote might suggest more nuanced rule systems if those should be needed (ignoring no/yes votes, setting thresholds manually or on the fly (in the model-predict API, etc.)

2. There will be 'mixed signals' from time to time for whatever reason (FP, FN) Aside from a serious malfunction where it does not recognize the start or end, there should should be at least two boundary points identified as 'not outside'

Possible simple overall rules:
1. Use model classification not votes to flag boundary spaces.
2. "Not outside" is a boundary space. (e.g. if 1=outside, then 0 is not-outside; if you defined the reverse, then the opposite is the case) 
2. If not two boundary points, return empty or error.
3. If two or more boundary points, take the outer two: return whatever is between the boundary points/"spaces". 

note: for brevity you could use |> and <| instead of STRING_START STRING_END, as those are unlikely to appear in a question about schedules.

Note: This is not an end-to-end byte-index model output, but it may be just as good for short sentences and a small fast model (and this few could be run in parallel on most systems). 

Note: Because this overall system will be for fuzzy matching (e.g. levenshtein) against a separate target database, off by one error (or words such as 'the' which may simply be stripped along with spaces when checking the database) may not be a problem). 

Note: one common sense note about interiors. 
The outside region will be on the whole stable and known (ways of asking when something is scheduled do not change much over time)
The inside, on the whole, is full of future unknowns (on the whole). But importantly it also contains very predictable standard things. If your office trains a model for this, much of the 'inside' will be the same things while 'on the whole' it will always contains novel and one-off content as well.

Note: I expect this is a significantly different type of modeling than classification (per se), and am not expecting to use any existing model for this.

Note: Strategies for handling the 'inside' (in training data and training mechanics) vary from making use of frequently repeating event names to using masking/drop-out/random (not-learned) strings, to ignoring "no-votes" (where 'no' means 'not-outside', so no-votes try to learn what is inside) and only count 'yes' votes the recognized outside) or a mix of these approaches. 
A mix of common event names and random-unique strings may be simplest. Dropout/making might be helpful too. Ignoring no-votes might be good IF there are no commonly repeated event names for a given case (but that is not MPV-1). 

Note: Finite-State-Machine-Layer
To some extent this is almost a kind of regex around an unknown problem-space. The questions will be highly regular (if in a number of forms)...is there way for TM clauses to be regex-elements? 
Could it work to have another layer where on top of detecting a given byte/token clause, randomly tried regex operations are tried regarding (and maybe other known clauses), and those regex-operations are voted on as well?

Q: Question: for training data formatting, each row would be a space-straddling span (word{space}word} with the outside=1 not-outside=0 lable?

Note: If full-head or full-tail is in the frame, the classification should always be: this is a boundary.

#### "Do I stutter?" Stuttering-Head-Tail Token/Byte Windows
A recurring question I keep coming back to is: Can the sweeping window be bigger by 2*N tokens... by using "stuttering" N-head and N-tail tokens?
E.g. If we start and end with
class=1 	|> |> When's the film? <| <|; |> |> When's the
class=1	|> |> When's the film? <| <|; > When's the film?
class=0 	|> |> When's the film? <| <|; When's the film? <|
class=0 	|> |> When's the film? <| <|; the film? <| <|

It may be case-by-case to up to results, but having a larger window might help with making context less narrow...

Using N pads (here N=2), the instead of the window being 2-tokens, it may become 2*N tokens. 
Could we have a larger token window while still having the score/prediction/classification/decision be about the 'focus' token, which is the space that is potentially a boundary-space?

36. Boolean-RegeX-Feature Granmo Machine Stacks/Layers: Boolean RegeX-FSM Layer 
(highly speculative)
Gramno-TM as Boolean Regular-Language State Machine:
Computable Granmo-TM space as Semi-Bounded Regular language?:

Genetic Algorithms & Decentralized Voting: Finite-State Machines + Granmo Machines (normally "TM"s)

Given that the ~results of regex operations can be (eventually) boolean values: 

If existing and well-demonstrated Granmo models are (Disjunctive Normal Form (DNF)) FST Boolean-feature learners that use boolean metadata about n-grams (the boolean match for an N-gram), such that Bag-of-Ngrams Gramno TM classifications models could be said to already be a minimal form of boolean-Regex derived metadata feature learning model: What other boolean regex-derived corpus/document metadata features could be seamlessly added to expand the reach of the boolean-metadata feature set? (see 'Standard N-Gram Positional Boolean Metadata' below.) What also simple/minimal features are low-hanging fruit, and what more derived but equivalent-type features could be tested?

Can we define the subset of regex in which TM-models normally live? e.g. 
- linear-time matching (as in Big-O accounting),
- decidable equivalence (decidability/definability is of value (including for audit/verification/testing/reproducibility), 
- minimal-DFA canonical hashing (hash verifiable same lexicon), 
- structural dead-clause detection (requiring that clauses can be satisfied in principle)

No:
- no ReDoS (guarded by AST source of potential new variants/mutations)
- no backreferences (e.g. to make character pattern repetition regular)
- no unbounded counters (no unbounded repetition of multi-char chunk; no unbounded int result;)


if 
clause = an intersection of regular languages, 
negated literals = complement, 
class vote threshold = accept-set over the product automaton 
is the learned model then a regular language?

Can adding regex to BOW/BONgrams TM classification a less extremely minimal version of the same traditional model, rather than adding anything 'new' to the architecture? 


Note:
Perhaps case by case, as this may be used for highly varied real life cases:
1. genomics
2. ascii NLP
3. unicode multi-language NLP
4. small doc
5. large doc
6. IoT data analysis
7. symbol is signal NLP
8. symbol is noise NLP
etc.

It is likely infeasible to prescribe a set of boundaries that are practical for all use-cases, but with human-language-NLP, probably various 'possible but expensive' operations will not end up being useful most of the time. It is also possible that none of this will be useful any of the time! But there may be something worth trying.

E.g. All-Positional pairs are quadratic, but 'All' might not be needed:
A. They might not be needed at all
B. Not all positions needed: It could be that one or some of the four end up be much more common (so the others are less important to check).
C. Any might be an indicator that others are or are not useful, such that you can prob-check quickly with one. (Or something similar in scenario where you are running multiple tests anyway, perhaps check one positional feature at a time (and none of them). 
D. Not all N-grams: This is something that maybe-could-be checked empirically: you might not need to do positional-feature checking on the entire lexicon (most of the time). E.g. In a normal distribution of n-gram frequencies if 1.5IRQ (or just simpler 1IQR) is fast and effective ~most of the time, that might cut down on the cost. (or, conversely, maybe the tails end up being more important, so just check those, etc.) (or, maybe high and low frequency are better to pair (vs. high and high or low and low))


Note: It could be (very speculative) a feature that could (in some cases) (potentially) boost extra-small models with deliberately small lexicons: E.g. N-gram size 2*4 is a lot smaller than N-grams sized 8. 

Maybe if the model is size-limited (embedded device, etc.) and N-gram-len=2 + positions is a compromise within the size constraints compared with Ngram-len=3 or 4.

Or in some cases maybe time or size isn't an issue and the only thing that matters is performance + Granmo-explainability... using a rack of Nvidia H100s will probably make most of the bloat less of an issue compared with a quadrillion parameter foundation model.


In terms of problem-space, I am not entirely sure that the scope management of these Part-1 (fixed standard regular language extensions) or part-2 (GA (AST genetic algo)) is definitionally different compared with the parameters of a normal GM-TM: If the parameters are too loose the result is big and slow. Come to think of it, that's what Prof. Skip Ellis tried to teach us about all computer science problem spaces; theoretical solutions and viable solutions are not identical: finding viable solutions is not trivial.

That said: THere should be general rules of thumb, guard rails, and tricks that will work. (again... assuming any of this has any value, which is a stretch).

Note:
for practicality it might useful to ~define
A. in theory a GM-TM is a regular language in terms of ability
B. in practicality a subset of regular language can be optimal
This might be why a safe starting point is the most minimal boolean-count-metadata (BOW-TM) (perhaps like the safe starting most minimal TM automata), 
A. most minimal is a (beautiful) kludge, not a definition of the problem/engine space
B. other features are both within the space and practical
C. more clearly defining the space will help to define the range of practical options


Idea: Defining a Semi-bounded Regular Language? 
(related to scope-scale and big-o(& NP-completeness...Turing Oracles?))




Is there a way for Granmo model clauses to be associated with regex components/elements?

- literals
- metacharacters
- character-classes
- quantifiers
- anchors
- groups and alternation: groups, capture groups, non-capture groups, alternation (logical OR matches A|B; Flag: this may be useful!)
- alternation
- escapes & predefined classes: Digit, word-char, whitespace, negated
- Lookarounds (Zero-Width Assertions): positive/negative lookahead/lookbehind

E.g. As a way to bridge part-analysis and analysis-parts with the effects on the entire document (which should include or be shaped by:
1. interaction between parts of analysis
2. properties of the whole document that are not found in isolated parts
3. (stretch-goal scope) potentially the ability for a decentralized modeling process to happen in a more decentralized way (e.g. not just abstract automata all in the ram of one device, but N devices in a network (e.g. measurement devices spread out over a field-research site).


Standardized Parts: State-Machine Meta-Data
While n-grams are a time-tested useful set of patterns, what other problem-space-subspaces are there?

1. Frequency: Bag/TFIDF (BOW, BO-Ngrams, Byte-Bag, etc.)
2. Sequence & Convolution Patterns
3. RegeX Meta-Data

(In theory, an ensemble model could use (for example non-false-positive) versions of all these types of patterns to try to better identify classes/class-labels?)

Modular RegeX (or Finite State Machine) Meta-Data:
For example, let's say we have two NLP classes. 
- A given regex search can return results that can be meta-data described either as binary 1/0 (something/nothing) or by int quantity, where a simple threshold may be drawn e.g. describing an intermediate between the median results of each class.

Metadata Features:

- A variety of perhaps 'standard metadata' items (such as phone number matches, email matches, date-format, html-tags, time in 24-hour format) can be checked along with genetic-algorithm style varied randomly tried binary or quantity regex queries (or perhaps the quantity based results could be automatically interpreted as binary given a threshold, so that all meta-data fields are binary.
Given a proverbial (or literal) table of meta-data X fields for each class, the Granmo-model system may be able to vote on class-identification relevance in a similar way to how BOW/TFIDF 'features' are voted on.


A. N-gram vs. Other
In a sense the RegeX idea here is a diversification of an existing singular mode: Gramno-TM is already good at using frequency based boolean metadata about n-grams. We can ask at least expansion questions:
1. Can we use other simple standard boolean metadata about the document?
2. Can we derive other boolean meta-data using N-grams (e.g. by making regex queries using those N-grams)?
E.g. if a regex pattern returns either something or nothing, that is already boolean. If the regex query returns a list, then that list containing more that Threshold T can be the binary filter, e.g. More than 0, or more than N (where various values of N can be tested for usefulness in comparing classes).


Standard N-Gram Positional Boolean Metadata:
For each pair of n-grams (A & B), there are two positional-metadata boolean values: (Any A found before Any B) and (Any B found before Any A)

- N-gram positional metadata: Does one n-gram occur before another? (boolean)



# 'standard metadata' items
( brainstorming notes )
- vowel patterns
- consonant patterns
- symbol patterns (could be a simple as a class with vs. without)
- capitalization patterns
- punctuation patterns
- text length patterns
- repeated character patterns (e.g. repeated symbols in hate-speech)

vs.
# making new regex based on n-gram table:
- Frequency may be the most useful meta-data signal about an n-gram

- n-gram 1 | n-gram N
- 


Architecture/Integration:
Could it work to have another layer where on top of detecting a given byte/token clause, randomly tried regex operations are tried regarding (and maybe other known clauses), and those regex-operations are voted on as well?


Sample Walkthrough 1:
With or without other types of features (e.g. Bag or N-gram frequencies)

#### Part 1: Feature Engineering
1.1 (MVP-1) Produce a table of boolean-results for a standard set of regex ~queries

Part 2. (MPV-1) Run Granmo-TM on the boolean features the same way that it would work for Bag-of-N-grams boolean features.


#### Sample Walkthrough 2:
With or without other types of features (e.g. Bag or N-gram frequencies)

Part 1: Feature Engineering
1.1 (MVP-1) Produce a table of boolean-results for a standard set of regex ~queries
1.2 (optional) Randomly generate other regex boolean features
option A. a one time process (there would be many useless clauses at the start)
option B. looped process to find the most useful queries while using a manageable set of clauses at a time.
1.3 Log useful queries found, to maybe add to the standard set to always try

Part 2. (MPV-1) Run Granmo-TM on the boolean features the same way that it would work for Bag-of-N-grams boolean features.


(Note: a genetic-algorithm farm for regex-queries require some kind of grammar generation framework 
- AST grammar framework for generation?
- 


37. Larger Structured-Extraction Process

Step 1: Detect Language (e.g. main eu languages)
- multi-class model?s
Step 2: Question-type classification
Step 3: Run model to extract item: Structured-Extraction (based on question-type)
Step 4: Levenshtein distance match to list of event names.
Step 5: Structured-Format 1
Step 7: Form-fill structured format to language to confirm question
Step 8: 

For 'structuring' unstructured text into specific structured fields:

A structured-extraction set of tools (I am guessing multiple models, not one), e.g. to extract a time-of-day or phone number, 1. a model to get starting byte, 2. a model to get ending byte 3. a simple extraction from-to bytes. 

mvp-1: not dealing with edge case expressions (quarter after, half past, half six, etc.)

'What's at 8?'
'When is {event}?

day of the week

day of month

month

month-day


38. Implicit vs. Explicit Ensemble
What kinds of features can be used in the same 'model' in various ways?
1. same flat pool of clauses and votes
2. parallel models with modified false-positive
3. other: stages?


N. convolutional levenshtein functions?
/////////////////////////////////////////////////////////////////////

TODO:

notes:
1. bow-tfidf 'flat' works
2. windows do not help, at least so far
3. clustering does not help, at least so far
4. Byte-Convolution-Streaming works


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

3. future steps / 
A. use the first-tests to figure out what more data we would like.
B. design and run more tests, including some 'deeper' runs on larger datasets, and test of more datasets
C. use those data to decide about adapting the module for particular uses
e.g. NLP language classification vs. potentially other data with different properties
