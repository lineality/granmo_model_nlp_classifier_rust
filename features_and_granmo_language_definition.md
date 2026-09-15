features_and_granmo_language_definition

# Feature Discovery & Language Definitions in Granmo Models
2026 09 10th Geoffrey Gordon Ashbrook


#### Feature-Discovery
One of the underlying, or over-arching, themes in this set of experiments and explorations, being in some respects narrow (if overly) and in other respects broad (if overly), is the topic of feature discovery and the agenda of expanding a possible, coherent, 'collaborating' set or family (or language) of features for one (narrow) problem-space or more (broad) problems more generally.

Narrow:
Starting from an NLP (natural language process) context, we will look at trying to make two additions, or two levels of additions to a 2018 'vanilla' Granmo Model (TM):


## Level 1: The Standard Library
Can extend the standard default feature space used by 2018-Granmo Models? This question is largely empirical.

Is this, or can we define this, extended standard feature set as an implicitly native possible feature set that was nascent already in the 2018-Granmo system definition, where the 2018-Granmo definition was (this is the question to investigate) more minimal than necessary? This question is more abstract but should feed into other followup empirical experiments.

The starting point for level-one (again, from an NLP perspective) is looking for a subset of RegeX operations that can be translated into 2018-Granmo-Model DFN bitwise-operation compatible features.

The question will then be framed as a language-definition question: Something like: can Granmo-model feature space be described as a formal language that is a super-set of the original 2018 feature-scope (and is this practical for any use-cases to improve model reach)?


#### Features & Equilibria
Another aspect of the question about feature discovery, feature quantity-scale (perhaps feature type), and perhaps related parameter settings, is the claim or statement in the 2018 paper that Granmo models show only an Global Minima approach and not a stochastic exploration of local minima. This may be a question of semantics around minima and other equilibria, but in tuning NLP Granmo models I have found that 'best' parameters cluster around different equilibria, suggesting (consistent with common sense) that no one pattern is all patterns (which might be a perspective at odds with the idea that a fundamentally confused NLP question-amalgam (e.g. classify this set of data from crazy people saying random things (labeled by the same or other crazy people), where no two people let along a quorum of people could agree on a given label per row or on what that label was supposed to mean, can ever have one single global minima). 

While in some ways mootly abstract, in other ways this 'how many equilibria' question relates directly to implemented and possible strategies for how to search through feature/parameter space to A. identify features and or parameters and B. to stop looking.


## Level 2:  

In level two we will take another step into the problem space of feature-discovery and ask whether there can be a native-compatible process of searching a larger feature space, with versions that may include a single Granmo-Model or N-other Granmo-models that perform feature-selection decisions.

The overall scheme, again from an NLP standpoint, is exploring an approved AST (abstract syntax tree) subset of RegeX type boolean-defined features. This may be where some of the 'language definition' steps connect the abstract with needed empirical test and design data.

### GA Genetic Algorithm:
One question here is whether the feature search can or should be formally or informally described as a Genetic Algorithm process, and or in terms of populations and mutations (if not other instrumentalist terms such as species and other feature equilibria).


A theme here is keeping feature quantity, and the resulting computation cost and model size, down. It would be a 'good problem to have' if the inherent/native compatible feature space of 2018-Granmo-models were so large that search/sort/narrowing from the abundance became the priority.


# Overall Framing:
Possibly depending on what results from empirical results, there are various ways this set of goals may be phrases, such as (possibly)

"Defining Granmo-Model Tsetlin-Machine feature-space as a formal regular-language subset"

"Identifying or defining FSM set overlap between 1. RegeX engines scope as FSM over input strings and 2. Tsetlin Automata as FSM over learning feedback."

"Using Disjunctive Normal Form (DNF) compatibility to define and expand feature scope of Granmo Models by defining Granmo Tsetlin-Machine Models as a specific formal regular language subset."

"Linear-Time Subset" vs. "DNF Bitwise compatible subset"




## Feature Space Management

Either a direct or indirect part of this definition or implementation is how the scale of features may be a factor/issue and or how feature-scale may specifically accommodated or managed (some of which may require or come from empirical results).

Possible factors, perhaps case by case are:
1. Whether quantity of features in a given Granmo-Model vote process causes performance issues other than time-speed. Is voting affected or impeded by either quantity of clauses or presence of bad clauses? (or is each clause separate?)

2. How empty or low performing (or other issues) features can be detected and pruned (more tree talk; ) (e.g. always fire, never fire, below as Document Frequency threshold, etc.)

3. Are there commonly cases where 'search' may take significant time, but the resulting model may be small and light and a good asset?

4. Is it possible or practical to have default pruning/pre-pruning rule such as for Quadratic-cost Positional Features (from A-exists & B-exists, to A before B exists and B before A exists) to only try to apply this to (something like) the top 25% DF (highest Document Frequency) N-grams/tokens/byte-sets.
(Bonzai models or modes?)

5. Are there some cases where a larger model may be desirable if it is more performant (e.g. where explainability justifies the use)?


# Feature Discovery, General & Specific: Definitions and Computability

To try to frame our problem-space (or what we do not know about our problem space) let's look at three example/case-study/items and a few adornments:

1. Granmo's 2018 paper I think includes the scope of nonlinear pattern modeling and deep-learning type pattern learning (given specific references to those that I do not interpret as placing those topics outside of the discussion). (Note: Looking at the 2018 paper alone is not meant to somehow be pedantically definitive, Granmo might have said something in more details or simply different elsewhere; the point is that me talking about non-linear patterns and deep-learning type flexibility are not necessarily outside the scope of this discussion where the main hub is Granmo-modeles (be they 2018-type or whatever he has invented in the years since).)

2. The conspicuous strangeness of Human-expert hand-made Tokenizer pre-processing layer for trillion-parameter multi-billion-dollar foundation attention models.

3. Byte-Tokenizer for Perseid Models

Accoutrements:
A. Sutton's 'Bitter Lesson' Observation that more data and statistics do a better job than (frequently hubris-ego coloured) human-expert-made model parts. (Granted, the whole short paper is rather casual and flippant and many various other points are nuanced, buried, and not automatically clear. I am not claiming that this point is either the only or the main point of the paper, but arguably this paper mostly agrees.)

B. The feature-discovery including raw-data pre-processing is a key part of
1. What makes end-to-end deep learning model difficult to dissect
2. What makes deep-learning models more effective: they make better features and find better types of patterns.

Dipole:
1. Do One Thing Well and Case-By-Case, vs.
2. A general framework or tool-set

My primary use-case/test-case for developing Granmo Models is for Natural Language Processing, and my primary happens to be English, using an ASCII character-set. But there is a tricky balance between an instrumentalist use of methods that 'juice' the results for today's case, vs. a perhaps slippery-slope of a model that works either more fundamentally or more generally. 



Semi-Rediculous Thought Experiment:
Is, or where is, there a fundamental disconnect between a bit/byte/semi-analogue raw data source and a binary decision-unit?
A. Is there a natural 256-bytes grammar?
B. Is there a natural 512-binary sub-byte grammar (boolean)?


1. Human-hand Binarizing the data seems absurd, though it may be a practical kludge to 'juice' the results for most everyday use-cases (for a particular use). 

2. What would it take to use granmo-models 'farther down the tool chain' so that instead of granmo-models being used only at the last binarized step, that granmo models are used for either more or all of the processing. (Or as discussed elsewhere is this a confusion of tools in the tool box. Does there need to be a strict separation of a bounded linear final inference process, while 'feature' exploration phases need to be in varous ways at various steps unbounded or less bounded?


# Artifact Discovery
Is "Feature" the wrong idea, or is it useful?
As in the classic example of the refracting telescope creating artifacts in the data that do not represent reality (two differently coloured planets where reality indicates one planet), has the piece-meal evolution of data-science and computer-science invented an artificially demarcated reification that we call "The Feature!" by way of cargo-cult sanctifying hand-imposed kludges into a workflow in order to find what we expect to find? 

For example, the best performing models are deep-learning models in which there may be no such distinction along the set of definable sub-processes by which "features" are "engineered" and passed in neat boxes from one homunculus to the next in a conspicuously anthropomorphic data factory. 

At a higher(?) general level are there feature-less problem spaces and also featured problem spaces? In various cases we probably do have situations where a tabular format of X and y columns is simply how the data naturally are. Or is that too a customary convention that water-to-a-fish has been assumed to be a 'separate' process. Is this not a natural extension of the 'Bitter Lesson' that the manual task of determining from raw data what are the features and how those raw signal should be formatted and pre-processed into neat labeled columns? Is this perhaps part of at least two mysteries:
1. The strange divide between symbolic and subsymbolic (where the once alpha-king-Minsky hulk-smashed any detested whispers of the subsymbolic)
2. The perhaps meatier fumble that from Whitehead through to so-called (but never explained 'symbols' on Turing's 'ribbon tapes' and on, 'symbol' has been a profoundly undefined emperor's new robe, casting the entire supposed dichotomy of 'new robe' vs. 'not new robe' into vapor. 
Is it time to call the bluff and confront the priest-class gate-keepers about their voodoo 'symbol' hoax, or will we punt for the next generation while the carnage results continue to rend? 

Q: Is there confusion about parts of the overall process in terms of what should and should not be binary and where, at a given point, a 'feature-set' comes from?

And yes, this discussion is perhaps a bit too silly and we should get down to brass tacks. But the design questions about how to interface a left hand full of pouring sand and a right hand full of binarized tabular inputs remains forked between:
A. Case by case, where any route and kludge will do.
B. The General Set of Tools and Terms



# Four Semi-Intersection Branches of Statistics
1. Bayesian
2. Fisher-Frequentist
3. Fully (or Partly?) Sub-symbolic
4. Granmo Synthesis: Game-Theory, Information Theory, Autonmata-Theory, Computational-Linguistics, distributed & social systems, Decision making, Etc.

Part of what I love about Granmo models is how 
1. Production-release Practical for strict (e.g. Rust) bitwise compute and auditiding
2. They represent a different approach
3. They compliment the overall set of tools and suggest a slightly less cargo-cult approach to the nature of patterns.



For example (at the real risk of flailing to a disconnected area, but I think this may be materially related), in Deborah M. Gordon's popular works (3 books as of 2026), she focuses on the use of interaction-metadata as signal for a distributed colony of ants. Perhaps related to the question of what is meant by 'feature' and 'feature-derivation', there are a number of interesting questions that come from the study of Ants. In what ways to ants 'learn' or 'find' or 'communicate' or 'distribute' or 'use' patterns, where we can restrict ourselves to the most conservative main behaviors of ants, including scouts: finding and communicating and unknown locations of semi-known resources; 

And there are other project-task contexts that may be useful to study as well, such as how ants self-delegate task-role decisions (how they decide what type of task to do and switch to when) in a distributed social orchestration that is decentrally coordinated.

'Real Time Dynamic Perception'
'Periodic Model Updating, (perhaps 'batch')'
'Continual Model Training, like "Reinforcement"'

As a task of using a pre-trained model (no pun intended) blurs into the tasks of making a new vision model or evaluating a potentially new or irregular source of data, especially where the process is colony-swarm-coordinated, the old questions about Features and feature-discovery may rise again more concretely. Given how quickly and auditably Granmo Models can be trained and deployed, I think Granmo Models are a natural fit for this area of real-world production tasks.

As part of framing out our problem space and definitions, it may be instructive to look at areas of adaptability. (As Lear's all licensed fool put it, "We'll set thee to school to an ant".)

Ants are extremely good at adapting within ranges of variation and within the domains of their actions; they are extremely resilient to a wide range of 'disturbance regimes' and extra-regime outliner-long-tail disturbances. But they do not 'adapt' to various tasks that "people" adapt to: ants do not learn to run businesses or on the fly set up new trade networks (though their half-programed niche often involves an expected web of exchange and symbiosis). (And at the same time, it often takes "people" decades, centuries, or millenia to 'learn' what to do and how to do it... and then frequently forgetting after all that work.) Ants do not overtly modify or study their own DNA. As may be demarcated by the traditional 'population-time vs. individual-time, and where a colony is effectively (or literally) one individual body that happens to be not always... uh ...physically continuous. 

The normal scope for looking at any model is probably more or less "How well does model-M do at MNIST in a lab?": score/miss; win/lose. I am deliberately trying to cast a larger net. With Granmo-Models it is (nearly) low-hanging-fruit to expand the set of 'modeling tasks' to include 
1. Collecting Datums 
1. Modifying Datasets
2. Writing/modifying code that modifies datasets
2. Writing/modifying code that manages types and uses of datasets (test, train, clean, prune, update, deprecate, etc.)
3. Writing/modifying code that evaluates models performance
3. Writing/modifying code that pre-trains models (test, train, validate, cross-validation, pruning; standard workflow space)
3. Writing/modifying code that batch retrains/updates models
3. Writing/modifying code that continually retrains/updates models
3. Writing/modifying code that predicts / performs inference
4. Use models to determine when to use a model
4. Use models to determine when to make a model
5. Use models to identify patterns to use for code (e.g. as context for what/how to write/modify)
5. Use models to identify tasks to start
5. Use models to identify tasks to end
5. Use models to identify tasks to be done
6. use models for (signal input) perception-detection
6. use models for (signal output) detection-perception



(N. And from the standpoint of definition behavior studies, using code and models to membrane-reinforce the integrity of the definitions and maintainable systems in place.)

And as to what the starting context is, a nested automata 'ant colony' might be a reasonable default starting set of functionalities. Note, there are at some forks that might represent design-choices, for example, 'general automata.' On the one hand ants make great use of the 'general mode' system, with the boot-strap exception of having a separate queen and (usually) periodically having separate (haploid) males. This might suggest a 'more ant than ants' method of allowing more task flexibility into a general-mode so that the colony is entirely general-mode (with "queen" being a general task, not an outlier role). Or, on the other hand, having periodic specialists might itself be a feature, so that the body of general-ants could have a larger repertoire of periodic specialists it can make and deploy (keeping the standard-ant slimmer). 

We should also make explicit (lest you think that this wooly tangent has already had too much lemonade to bear) where 'feature discovery' usually exists in biology/ants. This is to some extent a very pertinent high level topic, focus, and goal, not arcana or recreation (if those two are ever not already the same). Biological "evolution" (surely, like probability itself, not a topic that ever inspired disagreements among rational men...) is the original paradigmatic model for systematic technological invention and advancement. "Learning" occurs at the population level with an ongoing feature-discovery-exploration and "strict pruning" iteration model. It is this basal-distal hard-ware/software paradigm where (perhaps as in the above discussion of feature-discovery happening outside of the model (by mysterious 'hands of the hubristic creator'... or if that folly is the 'bitter lesson')) as with ants certain types of learning and adaptation are allowed to happen at the individual/cell/organ (or individual-colony) level, whereas other types of learning occur only at a macro-population level. 

In various species there has (probably for some time) been a marginal grey area where intelligent species such as bird-dinours, some cephelopoids, some mammals, for tens or hundreds of millions of years have marginally used tools and developed lifetime-learned skills. The general account (by "people") is that it was with an as yet not fully explained recent breakout that "people" (perhaps with significant historical language-based data-libraries in play) really doubled-down on 'in-lifetime-learning' and sought to very explicitly employ a broader range of feature-learning in-lifetime. (Yes, that is a gross oversimplification, and a topic that is (as of 2026) fuzzy.)

So, to define types of 'features' and 'feature discovery' for Granmo Models, we have some historical parameters that should be noted. How 'low' in the hardware direction are we going to mention or plan to act? Is the 'slow hardware iteration' model going to be (attempted to be) preserved or will learning be otherwise organized?




(And then of course, the above incomplete and artificially delineated list might be a class example of either A. an imperfect codified place to start or B. an arbitrary encoding of something that should be a dynamic set of 'data-features model-features and agile user-story task 'features''


### Language & Learning
One possible non-marginal policy decision (perhaps mirroring the biological decision on how much resource load to devote to a (potential) language platform, there are at least three general fork directions that Granmo architecture can take:
1. Stay minimal and actively avoid language-concept heavy loads
2. Develop a heavy 'attention mechanism' comparable system for a functional level of concept use
3. Find (or try to find) a thin-computer short-cut to enough coverage of firth concept space, staying lite and slim, but actively pursuing optimized terrain. 



# Feature Pre-Analysis:

1. Feature-collision check

2. Using 'Information' Signals:
Univariate Pre-Screening to Manage feature quantity/scale:
Univariate Analysis of Candidates: compute mutual information, chi-square, etc., between candidate regex features and class label prior to training. Set a threshold below which features are removed from the candidate-set.

3. 


Focusing on n-gram type features: (which may include space-delimited genomic sequence data)

# Defining Feature Scope that is already in 2018 Granmo Models

# Defining Extended Standard/Preset/Default Generalized Granmo-Model Regular Language Feature Space

# Defining Further Extended Feature-Discovery Space
1. Normal
2. Advanced, e.g. managing Combinatorial explosion for N-item positional tuples (A before B before C) - maybe powerful if navigable
3. Defining AST (or levels of AST)


# Other Questions:

## Krummholz Decision Trees 
(From biology, a Krumholz Tree is a tree structure that grows horizontally, as where above treeline conditions are too adverse to survive growing up.)

A further question here is whether either level 1 or level 2 may represent a kind of 'flat-decision-tree' type model, where the feature space is in some ways a branching set of conditions, but restricted such that it is expressible in a linear bitwise DFSNMT?PDQBach flat feature set with better audit and explainability and understandability properties than a dense decision tree.

(Per layer explainability?)



# Steps and Tasks

Level 1:
1. Define the feature-types already present in 2018-Granmo Models
2. Define the proposed extended standard feature set
3. Define what is excluded and how (e.g. lookback, wildcard, etc.)
4. Define modes and mechanisms for how feature-scale can be (e.g. by configuration parameter) managed.
5. Identify N clean and unambiguous NLP-classification datasets to test on.
6. Implement comparison batch-testing for performance examination
7. Test: Look at the test results to see if anything is observable.


Level 2:
1. Define the Abstract Syntax Tree for Granmo-Language
2. Define a simple feature search (if narrow)
3. Test: Look at the test results to see if anything is observable.
4. Define a larger feature search
5. Test: Look at the test results to see if anything is observable.



# Performance Tests
1. Standard comparison batch: engine-type/pre-processing type where (other (shared) parameters the same) 2018 vs. general-language versions can be compared (A. standard-extended-features B. ~GA Feature-discovery)
2. Test/compare N-gram sizes and pre-processing types
3. 
4. Size-Capped Small Feature Sets: 
Resource-constrained models/use-cases with smaller-models/fewer-clauses may be a case where an extended-type but still small-quantity(of clauses)/size-model can demonstrate a performance boost.
5. Examine specific new features for 'information-flow' value and/but also how they contribute to Granmo clause-vote model performance overall. 
- anchors

# Datasets for testing:
1. Language classification
- spam detection
(Maybe comments on IMDB dataset class definition issues.)
2. Genomic Sequence Classification Tests




# ?
Languages, Firth-Relationships, and Bitwise Logic

This question is an object: Is a "Granmo-Regular-Language" a stricter DFA-decidable set (without entropy or ratios), or is it a looser "boolean feature families that work with TMs"?

Assuming that features are boolean (the 2018 question of Grey-Scale MNIST representation is next) some relationships are positional but not all, and I suspect that not all need to be. 

On the one hand it makes sense that e.g. whole document metadata (such as document entropy or document compression-ratios) would not work the same way that comparing two single-literal byte-features in a boolean way booleans would work. But, on the other hand, does this distinction become notably fuzzier as we look at different cases? What is the computational restriction here, is it a restriction around a meta-data-calculation that can be A. done on a single linear pass and or B. performed using bitwise operations (or something like either of those?)?

For example: Various methods of calculating entropy and compression would be incompatible, but would any proxies be compatible? While we are used to thinking of patterns in terms of ASCII-BOW (e.g. 'cat'|'dog'), or to think or 'regex-type' operations as those useful for ASCII search, but in terms of raw bytes (or bits) the may be strange but legal relationships that are useful as patterns but outside of the word-token "symbol-cult" mindset. 

Part of this question may (or may not) be assuming a known-human-language use-context for RegeX-family operations, vs. a perhaps stranger family of more technically functionally-conjoined operations. 
This may or may not relate to a set of interconnected themes:
1. Oddities around the definition of 'deterministic' 
2. Language & formal system definition
3. Turing's 1938 Thesis involving Oracles and computability
3. (possibly NP-completeness -but maybe not; not pressing that but always fun to look)

Part of the troublesomeness of the C language was that it was/is too open to too many behaviors.

When it comes to modeling and pattern-finding, do we in a sense want to deliberately create a 'C' type situation where the model behavior itself is strictly formally defined and is (linguistically) well defined, but flexible enough that it is able to bend around as extremely nonlinear and dynamical patterns as possible, if with a 'language level' feature/clause explainability layer that says "this is what leads to a match" without narrowing the behavioral scope of the match more than it needs to.

Or, conversely, is 'C' the other kind of paradigmatic model for modeling whereby models need to be more like a 'Power of 10' Micro-Subset of possible C along with specific 'explainable' pattern types that it is allowed to match? 

Another aspect of this, which may or may not be ideal in terms of simple granmo-2018 explainability is stacking or layering.

As I understand it, RegeX ("language") cannot perform some logical calculations because it has no unbounded state, or rather: single step (un-stacked) regex-language cannot perform stateful operations(?). And this 'level definition' may be related to the starting question of feature-engineering and feature-discovery as a process of steps and layers.

This may be pedantic, but what would prevent one set of regex operations from identifying matches, and then another regex operation from directly or indirectly counting or otherwise exercising operators on those data as operands? 

Layer 1:
A "finite-state transducer" to convert matches to "marker" symbols.

Layer 2:
DFA over the marked string is a finite-state cascade (Kaplan & Kay 1994, Xerox two-level morphology) https://www.euppublishing.com/doi/10.3366/E1750124508000263 

And this layer-separation may also (or not) fit back into the 'C Troubles' context. It may be what we are looking for to find a 'language' that can be segregated into functional-levels, where on a single-level there are operations that it cannot do (fewer or no undefined behaviors) but that the same language when stacked can bend around a more diverse set of pattern templates. 

Procedural Modular Functionality.

And just as Granmo-Tsetlin machines can be fed one into another, language expressions may be (perhaps in a classic admixture of data and operation information) composed into still sequentially linear stacked orders of operations where temporally past layers are operands for the current layer's operations (or something).

Side Plot: The 2026 Zig-Rust (Bun) Memory-Safety
https://codeberg.org/ziglang/zig/issues/36237
Could this be somehow used to track memory safety? 
E.g. convert the compilation into a regular language (or something like that)? (probably extremely off topic, but interesting)


C vs. NASA vs. Regex vs. Granmo
Item to confirm:
'Power of 10' Rules:
- no recursion
- fixed upper bounds (no unbounded)
- no dynamic-memory-allocation
are either close to or are the definition points that delineate a Turing-Complete language/machine from a finite-state-language/machine

Such that the (possible, proposed) Granmo-Regular-Language may be similar or equivalent to Power-of-10 rules imposed on (individual layers for?) feature-extraction: 
1. One unbounded loop to read the next byte (?)
1. State fixed at start
2. One forward pass over bytes
3. Every 'counter' 'saturates' at a (predefined) constant.

Single-Pass Constant-Memory (Per Layer?) based definition.

Measures or proxies that fit within these bounds are fair game hens:

Shannon*-Entropy (not-regular)
Possible Proxies for Entropy: A. Bounded distinct byte values, B. Bounded identical byte values.

Compression-Ratio (not-regular)
Possible Proxies for Compression-Ratio:
Bounded occurrences of N targets within a window (may involve lookahead / matching groups)
Bounded Run-Length.

Digit Ratio > N (not-regular)
Possible Proxies for Digit-Ratio:
A 'grid/map' of bounded thresholds


TF-IDF >= θ (not-regular)
Possible Proxies for TF-IDF:
(Perhaps implying some layering (earlier counting step)), having IDF be a known constant, count(X) >= Ceiling of threshold(theta) / IDF(X)


Another example of the formality of Bounded values:
TODO: check this:
1. All properties of strings <= length-L is defined as regular (? finite set?)
2. 



How does it matter what is defined as part of the model, or 'architecture (including ensembles), vs. the "language"?

Turing-oracle related:
1. Training and discovery are unrestricted computation 
2. Inference is defined as regular

1. Overall Modeling Process Definition:
- Discovery
- Engineering
- Preprocessing
- Setting Bounds
- Orchestrating Layers
- Training

2. Inference-Language Definition:
(The language itself cannot set bounds, but it can operate within them.)


Whole doc metric, Whole corpus metric, and item vs. whole-doc-metric or whole-corpus-metric are permitted as bounded proxies (e.g. graded-thermometer bins) and using pre-discovered constants.


(Q: Multi-class classifier is inherently an ensemble?)


(WRONG: the 2018 Granmo paper may have used a similar "threshold encoding" for the gray-scale MNIST study)(but maybe applies to later granmo paper?)


* As a footnote, Turing (not one for naming things) is known to have developed a very similar "entropy" measure system for his WWII codebreaking work, and Turing and Shannon worked together at Bell-Labs during the war (which is not at all to say that Shannon (and his wife) did not work very hard at ironing out Shannon's landmark standard). Turing had a knack for co-discovering things, including, (of all things) the central limit theorem (and of course the more famous Church–Turing thesis and his 1936 paper on one of hilbert's challenges).




# 

- Original 2018 paper by Ole-Christoffer Granmo  https://arxiv.org/abs/1804.01508
- https://en.wikipedia.org/wiki/Hilbert%27s_problems 
- https://codeberg.org/ziglang/zig/issues/36237
- https://www.folger.edu/explore/shakespeares-works/king-lear/read/2/4/ 


///////////////

Q: the role of booleanization?


Q: The role of 'cost'
While 'cost' is often treated as something that pure theory does not need to include, I think it either should or can be considered more material in at least some contexts. Applied computer science, and science in general, is about finding and stacking optimization-gems. I do not think this lever-ratchet should be entirely excluded from theory or relegated to 'mere convenience'. 
One possible analogy/argument may be that 'catalyzing reactions' and 'finding reactivity' plays a role in biochemistry and ecology beyond 'allowing systems to be faster' as if there could be a planet of 'slower but identical life' using inorganic chemistry.  

If booleanization and linear bitwise operations allow for modular optimized functionality, even with tradeoffs, that might represent a significant ecosystem of interoperable functionality. 


Q: Booleanization and feature-space-search...
Q: Booleanization, granularity, and calculus
Q: Booleanization and 'Gamification'


Question: In a (((learn-apply)*stack)*N) type model, would the whole process need to run in order to tell the first/lower levels if they were coming up with useful encoding/representation/gamifications?
E.g. with a -gated-learning feature included, does this suggest a kind of continual-learning/reinforcement-learning type system? (or conversely a system that will find and settle on an equilibrium...
Or is this a kind of 'back-propagation' question, of what tells the previous layer if the change was effective? (Does adding alternating 'externalization' layers introduce (or just utilize) a way for a forward-propagation-only system to be shaped around a ~cost-function?






Notes:


I have yet to work the current power-of-10 turing, analogy pattern echo section into the writeup, but I want to ask first about the booleanization...

A few questions:

It seems like booleanization is key in a few ways... i'm not sure what all the main landscape features are

Q1. is it possible to somehow build a 'digital-stack' so that the booleanization is allowed to happen at or near the bottom (either bits or bytes) so that the 'forced-booleanization' (as with MNIST) does not need to happen.
This may be like the byte-tokenizer, the option where having either 256 booleanized bytes represented or 512 byte-bits represented, there can be a kind of native raw-data-input approach.

In ascii-land, bytes are usually characters. The goal of this bottom-up-approach (or one version of this approach) would be to be more like a deep hidden layer neural network,  to make voted-on feature-discovering be unabstracted from "modeling/training" (if at some expense of interpretability, though perhaps each layer is still interpretable boolean?), so in a kind of end-to-end stack of Granmo models, N layers would select what N-grams to use and possibly what expanded booleanized features to apply to them, etc. Or at least that's good for theory-discussion (and testing). 

To some extent this is an ad-absurdum example that is mostly useful ~rhetorically or for framing and perspective, but I suspect not entirely only that. For example, for ASCII it is a ridiculous thought experiment to decompose characters into bytes and then into bits, why 'go backwards' when we clearly want those higher level features, especially if we are operating in an n-gram type way. But what if the scenario is NLP but not ASCII? What if it is Japanese that might be just hiragana, or just romaji, or just kanji, or the whole soup of everything. 

And while the NLP context is the primary first-to-test model type, this study should at least try to be more general.


One of the questions might be around how much a "Granmo modeling system" needs have some type of architecture (or if in theory there could be a very meta-learning system that was slow but looked for whatever pattern).

The three-ish modes of "Granmo modeling system" that we have considered so far (all for ASCII NLP classification) are:
1. Non-Sutton: Hand made starting features for a flat model. (e.g. preprocessed BOW n-gram text)
a vanilla BOW N-gram-length=2 is more or less like this, where you lazily or greedily use all those n-grams

2. A two-phase system, search a larger space of N-gram type features and prune, then run train on those features, e.g. starting searching n-gram-length=8 but very strictly pruning to reduce to less than a raw ngram-length=3 would be (possibly with some pre-test-modeling to test the features as much as possible).

3. Some kind of stacked system, maybe analogous to non-ascii-language, where the 'feature' 'encoding' (if such is not a fictional reification cargo cult 'isomorphic mimicry' of the ASCII-BOW workflow) is done entirely by either a separated pipeline step or by layers within an 'end-to-end' stack of modular Granmo model layers.

E.g. Due to the formalities of 'booleanization', the search for 'how to preprocess' might be something that one Granmo model can learn how to do, but that it would need to be done manually before feeding the original text into the next layer... or not. I am not sure about this set of aspects of Granmo models and the roles of Booleanization.

Or is the question somewhat one of cost and optimization, that something could be done using modular layers but it would be time-consuming?

This might also fold back into the question of interpretability, and possibly on N levels. Could it be that for both optimization of inference and for explainability that models in general (yes, I am thinking about the strange chimera of trillion-parameter 'end to end' Transformer models that have a hand-made pre-processing tokanizer-model/pipeline, duct-taped to the quarterized neck-hole of the Transformer-model) 
Would a more general-organic 'stack' of layers be a layering of two-types including externalization(see Object-Relationship-Spaces obsession with externalized project-object state) (which may or may not go back to the Transformer-evolution question of encoder-decoder vs. decoder-only) 
e.g.
A stack of N pairs of 
1st. Feature-Encoding/Representation-finder
2nd. Feature-Encoding/Representation-implementer

where the then-encoded data is handed off to the next layer,

and where during "inference" you remove the Feature-Encoding-finder parts and instead stack the Feature-Encoding-implementer, where either the final layer is also the final format, or an otherwise final layer is added.

Can or should there be better more general descriptions than perhaps overly model-specific terms such as "intermediate representations"





Q2. Maybe something about a computation level model needing to operate on booleans (as perhaps a turing tape (either Turing complete or smaller) might need to translate into boolean operations?

Q3. That the tradeoffs for modeling targets and types of modeling using a 'boolean first' approach should be more explicit, 
A. simple boolean 'features' for a simple small 'flat' model (target features need to be booleanized)
B. more elaborate strategies can make bucket-gradation systems
C. a more configurable type of modular/distributable model system?
D. possibly  the 'start low' the bits and bytes are already booleanized?
