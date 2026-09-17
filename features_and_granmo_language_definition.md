features_and_granmo_language_definition

# Feature Discovery & Language Definitions in Granmo-Automata-Machine-Systems
2026 09 10-16th Geoffrey Gordon Ashbrook


#### Feature-Discovery
One of the underlying, or over-arching, themes in this set of experiments and explorations, being in some respects narrow (if overly) and in other respects broad (if overly), is the topic of feature discovery and the agenda of expanding a possible, coherent, 'collaborating' set or family (or language) of features for one (narrow) problem-space or more (broad) problems more generally.

The term 'Feature-Discovery' likely refers to a heterogenous set of problem spaces, in particular
1. Data and feature preprocess and engineering before model-training, vs.
2. The 'feature-discovery' that results from the clause-creation voting process. 

One issue is that from a theory and principle standpoint we do not yet know if there is compelling 'general-rule' way to pick a side between these positions:
1. There are no inherent differences in 'types' of processing all through the pipeline from raw-input to final output (and in theory one correctly generalized process will account for them all).
2. There is only one distinction, between 'during-training-feature-discovery' vs. 'before-training-feature-discovery'
3. Distinctions come from artifacts of approaches, not from inherent 'phase' differences in data-transformation.
4. There are various types of formal processing-type and processing-phase distinctions in principle and no single 'machine-system' can do them all. 
Etc.

To make sure this distinction does not get lost, the sequentially first set of experimental targets is around during-training-feature-discovery in a context of defining a Granmo-Language. But the larger context is important in terms of what design decisions are clearly for what in reference to or for combination with what.


The context-discussion for this study is going to get, to the taste of some, perhaps a bit too philosophical and broad, but the approach taken here is to measure twice (or several dozen times) and cut once: The focus will return to both most-immediate narrowest experiments and scope and importantly to the breadcrumb trail of next experiments and where those should proactively push (in the absence of future direction-directing data). Mapping out the problem space is here considered concurrently important and in an interactive feedback process with incrementally designing experiments to get better problem-space maps, but also (while to some extend subsidizing modeling for the sake of modeling) choosing to have a specific concrete applied-STEM agenda for what project-tools these models can inform, and then this applied layer joins the virtuous feedback cycle of better data leading to better models leading to better tools, etc. etc. (And lest you think this paragraph to have been too roundly round, this is directly applicable to the design of all three parts of those interlocking loops: models, data, tools).


"Feature-descovery and engineering" and "language-definition" are inter-twined but to some extent remain two different areas within Granmo-Automata-Machine-Systems, and to some extend represent two or more goals of this study. A primary goal is looking at 'language definitions' to possibly define increased scope for otherwise mostly unchanged flat (one-level) 2018-type Granmo-Automata-Machine-Systems for NLP classification. But part of that problem space, and the question of defining what a Granmo-Automata-Machine-System is (including what is considered part of it and what is considered outside of it), is looking at how, if, and whether a Granmo-Automata-Machine-System can or should include "Feature-descovery and engineering" on various levels. Part of why I add the term 'system' into my descriptive phrase "Granmo-Automata-Machine-Systems," is that in various cases there may be a larger overall architecture and pipeline needed for a whole process where, notably, a flat-one-layer Granmo-Automata-Machine(System) may optionally be employed in just one or perhaps in multiple places, and potentially for everything in the system in some cases. But part of defining various parts of this overall system-space is looking at where definitions vary. For example, the rules (possibly definable as a "language") for automata testing of data X-feature metadata features/properties are local to that type of operation in the system; for example a methods of ~n-gram/data-gram pruning that is incompatible with that specific automata-vote process may simply need to occur outside and prior within the overall system of various processes.

This will also be a fractally self-similar investigation where initial exploratory tests and data will be needed to inform the priorities or sanities of what to explore later on.



### Narrow:
Starting from an NLP (natural language process) context, we will look at trying to make two additions, or two levels of additions to a 2018 'vanilla' Granmo-Automata-Machine-System (TM):


## Level 1: The Standard Library
Can extend the standard default feature space used by 2018-Granmo-Automata-Machine-Systems? This question is largely empirical.

Is this, or can we define this, extended standard feature set as an implicitly native possible feature set that was nascent already in the 2018-Granmo system definition, where the 2018-Granmo definition was (this is the question to investigate) more minimal than necessary? This question is more abstract but should feed into other followup empirical experiments.

The starting point for level-one (again, from an NLP perspective) is looking for a subset of RegeX operations that can be translated into 2018-Granmo-Automata-Machine-System DFN bitwise-operation compatible features.

The question will then be framed as a language-definition question: Something like: can Granmo-Automata-Machine-System feature space be described as a formal language that is a super-set of the original 2018 feature-scope (and is this practical for any use-cases to improve model reach)?


#### Features & Equilibria
Another aspect of the question about feature discovery, feature quantity-scale (perhaps feature type), and perhaps related parameter settings, is the claim or statement in the 2018 paper that Granmo-Automata-Machine-Systems show only an Global Minima approach and not a stochastic exploration of local minima. This may be a question of semantics around minima and other equilibria, but in tuning NLP Granmo-Automata-Machine-Systems I have found that 'best' parameters cluster around different equilibria, suggesting (consistent with common sense) that no one pattern is all patterns (which might be a perspective at odds with the idea that a fundamentally confused NLP question-amalgam (e.g. classify this set of data from crazy people saying random things (labeled by the same or other crazy people), where no two people let along a quorum of people could agree on a given label per row or on what that label was supposed to mean, can ever have one single global minima). 

While in some ways mootly abstract, in other ways this 'how many equilibria' question relates directly to implemented and possible strategies for how to search through feature/parameter space to A. identify features and or parameters and B. to stop looking.






## Level 2:  

In level two we will take another step into the problem space of feature-discovery and ask whether there can be a native-compatible process of searching a larger feature space, with versions that may include a single Granmo-Automata-Machine-System or N-other Granmo-Automata-Machine-Systems that perform feature-selection decisions.

The overall scheme, again from an NLP standpoint, is exploring an approved AST (abstract syntax tree) subset of RegeX type boolean-defined features. This may be where some of the 'language definition' steps connect the abstract with needed empirical test and design data.

### GA Genetic Algorithm:
One question here is whether the feature search can or should be formally or informally described as a Genetic Algorithm process, and or in terms of populations and mutations (if not other instrumentalist terms such as species and other feature equilibria).


A theme here is keeping feature quantity, and the resulting computation cost and model size, down. It would be a 'good problem to have' if the inherent/native compatible feature space of 2018-Granmo-Automata-Machine-Systems were so large that search/sort/narrowing from the abundance became the priority.


# Overall Framing:
Possibly depending on what results from empirical results, there are various ways this set of goals may be phrases, such as (possibly)

"Defining Granmo-Automata-Machine-System feature-space as a formal regular-language subset"

"Identifying or defining FSM set overlap between 1. RegeX engines scope as FSM over input strings and 2. Tsetlin Automata as FSM over learning feedback."

"Using Disjunctive Normal Form (DNF) compatibility to define and expand feature scope of Granmo-Automata-Machine-Systems by defining (what part or whole exactly? of) Granmo Tsetlin-Machine Models as a specific formal regular language subset."

"Linear-Time Subset" vs. "DNF Bitwise compatible subset"




## Feature Space Management

Either a direct or indirect part of this definition or implementation is how the scale of features may be a factor/issue and or how feature-scale may specifically accommodated or managed (some of which may require or come from empirical results).

Possible factors, perhaps case by case are:
1. Whether quantity of features in a given Granmo-Automata-Machine-System vote process causes performance issues other than time-speed. Is voting affected or impeded by either quantity of clauses or presence of bad clauses? (or is each clause separate?)

2. How empty or low performing (or other issues) features can be detected and pruned (more tree talk; ) (e.g. always fire, never fire, below as Document Frequency threshold, etc.)

3. Are there commonly cases where 'search' may take significant time, but the resulting model may be small and light and a good asset?

4. Is it possible or practical to have default pruning/pre-pruning rule such as for Quadratic-cost Positional Features (from A-exists & B-exists, to A before B exists and B before A exists) to only try to apply this to (something like) the top 25% DF (highest Document Frequency) N-grams/tokens/byte-sets.
(Bonzai models or modes?)

5. Are there some cases where a larger model may be desirable if it is more performant (e.g. where explainability justifies the use)?


# Feature Discovery, General & Specific: Definitions and Computability

To try to frame our problem-space (or what we do not know about our problem space) let's look at three example/case-study/items and a few adornments:

1. Granmo's 2018 paper I think includes the scope of nonlinear pattern modeling and deep-learning type pattern learning (given specific references to those that I do not interpret as placing those topics outside of the discussion). (Note: Looking at the 2018 paper alone is not meant to somehow be pedantically definitive, Granmo might have said something in more details or simply different elsewhere; the point is that me talking about non-linear patterns and deep-learning type flexibility are not necessarily outside the scope of this discussion where the main hub is Granmo-Automata-Machine-Systemes (be they 2018-type or whatever he has invented in the years since).)

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

My primary use-case/test-case for developing Granmo-Automata-Machine-Systems is for Natural Language Processing, and my primary happens to be English, using an ASCII character-set. But there is a tricky balance between an instrumentalist use of methods that 'juice' the results for today's case, vs. a perhaps slippery-slope of a model that works either more fundamentally or more generally. 



Semi-Rediculous Thought Experiment:
Is, or where is, there a fundamental disconnect between a bit/byte/semi-analogue raw data source and a binary decision-unit?
A. Is there a natural 256-bytes grammar?
B. Is there a natural 512-binary sub-byte grammar (boolean)?


1. Human-hand Binarizing the data seems absurd, though it may be a practical kludge to 'juice' the results for most everyday use-cases (for a particular use). 

2. What would it take to use Granmo-Automata-Machine-Systems 'farther down the tool chain' so that instead of Granmo-Automata-Machine-Systems being used only at the last binarized step, that Granmo-Automata-Machine-Systems are used for either more or all of the processing. (Or as discussed elsewhere is this a confusion of tools in the tool box. Does there need to be a strict separation of a bounded linear final inference process, while 'feature' exploration phases need to be in varous ways at various steps unbounded or less bounded?


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
4. Granmo Synthesis: Game-Theory, Information Theory, Automata-Theory, Computational-Linguistics, distributed & social systems, Decision making, Etc.

Part of what I love about Granmo-Automata-Machine-Systems is how 
1. Production-release Practical for strict (e.g. Rust) bitwise compute and auditing
2. They represent a different approach
3. They compliment the overall set of tools and suggest a slightly less cargo-cult approach to the nature of patterns.



For example (at the real risk of flailing to a disconnected area, but I think this may be materially related), in Deborah M. Gordon's popular works (3 books as of 2026), she focuses on the use of interaction-metadata as signal for a distributed colony of ants. Perhaps related to the question of what is meant by 'feature' and 'feature-derivation', there are a number of interesting questions that come from the study of Ants. In what ways to ants 'learn' or 'find' or 'communicate' or 'distribute' or 'use' patterns, where we can restrict ourselves to the most conservative main behaviors of ants, including scouts: finding and communicating and unknown locations of semi-known resources; 

And there are other project-task contexts that may be useful to study as well, such as how ants self-delegate task-role decisions (how they decide what type of task to do and switch to when) in a distributed social orchestration that is decentrally coordinated.

'Real Time Dynamic Perception'
'Periodic Model Updating, (perhaps 'batch')'
'Continual Model Training, like "Reinforcement"'

As a task of using a pre-trained model (no pun intended) blurs into the tasks of making a new vision model or evaluating a potentially new or irregular source of data, especially where the process is colony-swarm-coordinated, the old questions about Features and feature-discovery may rise again more concretely. Given how quickly and auditably Granmo-Automata-Machine-Systems can be trained and deployed, I think Granmo-Automata-Machine-Systems are a natural fit for this area of real-world production tasks.

As part of framing out our problem space and definitions, it may be instructive to look at areas of adaptability. (As Lear's all licensed fool put it, "We'll set thee to school to an ant".)

Ants are extremely good at adapting within ranges of variation and within the domains of their actions; they are extremely resilient to a wide range of 'disturbance regimes' and extra-regime outliner-long-tail disturbances. But they do not 'adapt' to various tasks that "people" adapt to: ants do not learn to run businesses or on the fly set up new trade networks (though their half-programed niche often involves an expected web of exchange and symbiosis). (And at the same time, it often takes "people" decades, centuries, or millenia to 'learn' what to do and how to do it... and then frequently forgetting after all that work.) Ants do not overtly modify or study their own DNA. As may be demarcated by the traditional 'population-time vs. individual-time, and where a colony is effectively (or literally) one individual body that happens to be not always... uh ...physically continuous. 

The normal scope for looking at any model is probably more or less "How well does model-M do at MNIST in a lab?": score/miss; win/lose. I am deliberately trying to cast a larger net. With Granmo-Automata-Machine-Systems it is (nearly) low-hanging-fruit to expand the set of 'modeling tasks' to include 
1. Collecting Datums 
2. Modifying Datasets
3. Writing/modifying code that modifies datasets
4. Writing/modifying code that manages types and uses of datasets (test, train, clean, prune, update, deprecate, etc.)
5. Writing/modifying code that evaluates models performance
6. Writing/modifying code that pre-trains models (test, train, validate, cross-validation, pruning; standard workflow space)
7. Writing/modifying code that batch retrains/updates models
8. Writing/modifying code that continually retrains/updates models
9. Writing/modifying code that predicts / performs inference
10. Use models to determine when to use a model
11. Use models to determine when to make a model
12. Use models to identify patterns to use for code (e.g. as context for what/how to write/modify)
13. Use models to identify tasks to start
14. Use models to identify tasks to end
15. Use models to identify tasks to be done
16. use models for (signal input) perception-detection
17. use models for (signal output) detection-perception



(N. And from the standpoint of definition behavior studies, using code and models to membrane-reinforce the integrity of the definitions and maintainable systems in place.)

And as to what the starting context is, a nested automata 'ant colony' might be a reasonable default starting set of functionalities. Note, there are at some forks that might represent design-choices, for example, 'general automata.' On the one hand ants make great use of the 'general mode' system, with the boot-strap exception of having a separate queen and (usually) periodically having separate (haploid) males. This might suggest a 'more ant than ants' method of allowing more task flexibility into a general-mode so that the colony is entirely general-mode (with "queen" being a general task, not an outlier role). Or, on the other hand, having periodic specialists might itself be a feature, so that the body of general-ants could have a larger repertoire of periodic specialists it can make and deploy (keeping the standard-ant slimmer). 

We should also make explicit (lest you think that this wooly tangent has already had too much lemonade to bear) where 'feature discovery' usually exists in biology/ants. This is to some extent a very pertinent high level topic, focus, and goal, not arcana or recreation (if those two are ever not already the same). Biological "evolution" (surely, like probability itself, not a topic that ever inspired disagreements among rational men...) is the original paradigmatic model for systematic technological invention and advancement. "Learning" occurs at the population level with an ongoing feature-discovery-exploration and "strict pruning" iteration model. It is this basal-distal hard-ware/software paradigm where (perhaps as in the above discussion of feature-discovery happening outside of the model (by mysterious 'hands of the hubristic creator'... or if that folly is the 'bitter lesson')) as with ants certain types of learning and adaptation are allowed to happen at the individual/cell/organ (or individual-colony) level, whereas other types of learning occur only at a macro-population level. 

In various species there has (probably for some time) been a marginal grey area where intelligent species such as bird-dinours, some cephelopoids, some mammals, for tens or hundreds of millions of years have marginally used tools and developed lifetime-learned skills. The general account (by "people") is that it was with an as yet not fully explained recent breakout that "people" (perhaps with significant historical language-based data-libraries in play) really doubled-down on 'in-lifetime-learning' and sought to very explicitly employ a broader range of feature-learning in-lifetime. (Yes, that is a gross oversimplification, and a topic that is (as of 2026) fuzzy.)

So, to define types of 'features' and 'feature discovery' for Granmo-Automata-Machine-Systems, we have some historical parameters that should be noted. How 'low' in the hardware direction are we going to mention or plan to act? Is the 'slow hardware iteration' model going to be (attempted to be) preserved or will learning be otherwise organized?




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

# Defining Feature Scope that is already in 2018 Granmo-Automata-Machine-Systems

# Defining Extended Standard/Preset/Default Generalized Granmo-Automata-Machine-System Regular Language Feature Space

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
1. Define the feature-types already present in 2018-Granmo-Automata-Machine-Systems
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





* As a footnote, Turing (not one for naming things) is known to have developed a very similar "entropy" measure system for his WWII codebreaking work, and Turing and Shannon worked together at Bell-Labs during the war (which is not at all to say that Shannon (and his wife) did not work very hard at ironing out Shannon's landmark standard). Turing had a knack for co-discovering things, including, (of all things) the central limit theorem (and of course the more famous Church–Turing thesis and his 1936 paper on one of hilbert's challenges).




# 

- Original 2018 paper by Ole-Christoffer Granmo  https://arxiv.org/abs/1804.01508
- https://en.wikipedia.org/wiki/Hilbert%27s_problems 
- https://codeberg.org/ziglang/zig/issues/36237
- https://www.folger.edu/explore/shakespeares-works/king-lear/read/2/4/ 



# Models, Learning, Features, and Adaptations:

The adaptability of ants in the face of highly dynamic conditions and environments may be an interesting concrete area where we can compare common problem-spaces in computer science and in data-science. 

Production-computer-science has long struggled with applying theoretically beautiful abstractions (that we love dearly and hope to be able to focus on) to the challenging dynamics of reality (which the human mind frequently seems to be repelled by). From the disquieting success of ELIZA, to the disquieting failure of SHRDLU, to the "snake oil" and "pennance" of P.J. Plauger, to the beard trimming of Holzmann, to the rise of Rust, to the perplexing delays in recognizing cybersecurity, to the perplexing delays in recognizing production data science, the 'ant-adaptation' problem space of carrying out 'lower-level' engineering tasks in a dynamic and disturbance-regime characterized set of environments with sufficient equilibria and minima to be able to maintain the enterprise is (if boring to people) a both a significant area and perhaps an area that may highlight significant domains within 'learning and adaptation' that are not overwhelmed by the higher-level curiosities and fantasies of language and exotic consciousness and super-mega-5000-diety-tier-AI. For example, it may be that in the 'boring' realm of C-type language basic utility code, that for resilience there should or could be some level of element of 'learning' to cover the production edge-cases. As of 2026 this 'learning' is the job of 'expert humans who hand craft the features and rules'(that should sound familiar), and for various reasons this learning generally fails to be achieved or maintained and the project goes extinct. But so far there is little discussion of 'low level learning' being inherently a part of low level software development (as in being inherently part of either the code compilation (and language) stack or a part of the deployed software itself, or both), or of low-level-adapation as an 'interesting' or valuable problem-space within Data-Science AI/ML (where discussions usually begins and ends circumambulating MNIST, IRIS, IMDB, etc.) (or in 2026 where thugs chant 'gen-ai!' and 'bitcoin!' and 'IPO' and 'Bro!' between ritual gang acts of violence and intoxication; almost enough to make one teary eye'd for halcyon days of early image net where, yes people were entirely wrong to poo poo subsymbolic learning in their group-monologues, but at least most of them were able to read).

This may be a stretch and erroneous, but when using a system like Granmo's the problem space of adaptability of basic low level processes to dynamic environs the set of issues at least seem much closer together and coherent than trying to abstractly compare C (or zig or odin or Rust) an on embedded device with several gigabytes of python-environment spaghetti (be that sklearn, NLTK, or Keras-Tensorflow, or PyTorch, etc. where the trying to argue that there is an inherent shared native set of needs and operations sounds more like Kurzweil singularity speculation or a 1990's Complexity-Sciences Theory of Everything (note: neither of those areas should be entirely ignored, but also neither are so far directly relevant to writing branch-less control-flow). 


2. Models and language
Models and language are two areas (maybe two areas?) that only have arm-wavy and niche dependent descriptions of. 

As Debora M. Gordon succinctly puts it, Models are often used differently by applied engineering-type STEM people compared with biology-pure-research people, though the grey area is, I think, interesting.

Using models to 
1. do stuff
2. make better models
3. get better data and feedback

Perhaps yet again harkening back to Ashby and Snow,
these three areas arguably apply to both 'engineers' and 'pure researchers' 


Part of our scope question here is about how all of these might affect a particular model architecture, e.g.
1. Where the scope is extremely narrow and focused only on doing something in the simplest slimmest way possible (In Bill Hall's delightful phrase "Just do the dumb thing." ( "Ginger Bill" Hall: Quantum Physicist and creator of the Odin language))
2. Where either the model or the model process is multi-step, e.g. involving using separate model-modules to inform and design the ~final model design and configuration.




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









A few questions:

It seems like booleanization is key in a few ways... i'm not sure what all the main landscape features are

Q1. is it possible to somehow build a 'digital-stack' so that the booleanization is allowed to happen at or near the bottom (either bits or bytes) so that the 'forced-booleanization' (as with MNIST) does not need to happen.
This may be like the byte-tokenizer, the option where having either 256 booleanized bytes represented or 512 byte-bits represented, there can be a kind of native raw-data-input approach.

In ascii-land, bytes are usually characters. The goal of this bottom-up-approach (or one version of this approach) would be to be more like a deep hidden layer neural network,  to make voted-on feature-discovering be unabstracted from "modeling/training" (if at some expense of interpretability, though perhaps each layer is still interpretable boolean?), so in a kind of end-to-end stack of Granmo-Automata-Machine-Systems, N layers would select what N-grams to use and possibly what expanded booleanized features to apply to them, etc. Or at least that's good for theory-discussion (and testing). 

To some extent this is an ad-absurdum example that is mostly useful ~rhetorically or for framing and perspective, but I suspect not entirely only that. For example, for ASCII it is a ridiculous thought experiment to decompose characters into bytes and then into bits, why 'go backwards' when we clearly want those higher level features, especially if we are operating in an n-gram type way. But what if the scenario is NLP but not ASCII? What if it is Japanese that might be just hiragana, or just romaji, or just kanji, or the whole soup of everything. 

And while the NLP context is the primary first-to-test model type, this study should at least try to be more general.


One of the questions might be around how much a "Granmo-Automata-Machine-Systeming system" needs have some type of architecture (or if in theory there could be a very meta-learning system that was slow but looked for whatever pattern).

The three-ish modes of "Granmo-Automata-Machine-Systeming system" that we have considered so far (all for ASCII NLP classification) are:
1. Non-Sutton: Hand made starting features for a flat model. (e.g. preprocessed BOW n-gram text)
a vanilla BOW N-gram-length=2 is more or less like this, where you lazily or greedily use all those n-grams

2. A two-phase system, search a larger space of N-gram type features and prune, then run train on those features, e.g. starting searching n-gram-length=8 but very strictly pruning to reduce to less than a raw ngram-length=3 would be (possibly with some pre-test-modeling to test the features as much as possible).

3. Some kind of stacked system, maybe analogous to non-ascii-language, where the 'feature' 'encoding' (if such is not a fictional reification cargo cult 'isomorphic mimicry' of the ASCII-BOW workflow) is done entirely by either a separated pipeline step or by layers within an 'end-to-end' stack of modular Granmo-Automata-Machine-System layers.

E.g. Due to the formalities of 'booleanization', the search for 'how to preprocess' might be something that one Granmo-Automata-Machine-System can learn how to do, but that it would need to be done manually before feeding the original text into the next layer... or not. I am not sure about this set of aspects of Granmo-Automata-Machine-Systems and the roles of Booleanization.

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


-> Better-Model -> Better-Data -> (loop)
"tools"
-> Better-Functionality -> Better-Feedback/Testing -> (loop)


# The Echo: 
Regular Language definition for Booleanization



An intersection between
- Regular Language Definition
- Turing Completeness Definition
- Holzmann/NASA Functionality Exclusion
- Computability/Optimization
- Granmo-Automata-Machine-System Inference Definition



## Semi-Overal, Mirror, Analogy, or Echo Areas:


## Tentative Pattern Areas:


## Questions:
- Single Layer vs. Multi-Layer Granmo-Automata-Machine-Systems
- Rules & Effects of Booleanization
- "End to end" feature discovery in A. multi-step/phase process, B. contained in a single (leveled) model
- "Externalization Layers"
- Forward and backward propagation given pairs of 'representation' and 'externalization' layers.
- Gated learning and cost-function definition
- feature-analysis in Re-training and continuous training


## Result Planning/Design Ideas:





Notes:

power-of-10 turing, analogy pattern echo \

# The Echo
I am using the term 'Echo' here because I have no interest in a "realist" mysterious theory of everything. In my experience, mirages of centers are things best avoided. There seem to be a strange number of, how to say it, parallel reverberations of similar themes coming from different disciplines. We should try to look at and learn from this as accurately as possible and not force it into a recreational numerological artifact.

Note the term 'bounded' is not simplistic here, it repeats in various ways and places. (e.g. Rules for what is and is not permissible as 'unbounded' for RegeX-subset, and the Two-Rules for Bounded/Unbounded loops from Holzmann.


- Regular Languages
- Finite State Machines
- DFA
- unbounded C, vs. restricted Hollzman-power-of-10-C
- Automata theory
- Turing, completeness, 
- Turing oracles
- Finite State Machine
	- RegeX
	- Tsetlin-Machine ?
- Finite Automaton

- parallelization/re-integration
- externalization of state (my own obsession)

(Note: I have worked on disambiguating some of the definitions in Holzeman's framing of 'Modes and Cases': see
1. https://github.com/lineality/modes_and_case_handling/blob/main/mode_case_handling_framework_summary.md 
2. https://github.com/lineality/modes_and_case_handling 

Overlap/Echo
- Overlap/Echo between Holzman Subset of C and "regularness" definition for language-theory


Possible equivalance (again, not interesting in chasing down hard equivalance):
- Power of 10 -> booleanizer, ~equivalent to regular-language restrictions
- 
- 

Memory allocation and memory ~validation? seems to be an important overall theme: no-dynamic allocation is obviously not "Rust memory rules" or "fil-C" etc., it seems aligned. (maybe)

- Rule: no dynamic allocation after initialization


The 2026 Memory-State Debate in Zig-Rust-Odin-C(e.g. Fil-C)


- Odd that Entropy-calculation is outside of this


Side question:
- In some ways these 'rules' are functional, but in other ways these rules may be arbitrary.
- If only for research purposes, would it be possible to make and test the performance of an unbounded ~spaghetti-C Granmo-machine that was able to exhibit shapes and behaviors that were less defined?
- 

..

"Flat vs. Representation": Chicken & Egg problem for pre-booleanized inputs to a 'Flat' process. 


note: most of these changes are at the flat-level...
- C was reduced
- 'regular' language is reduced, and Granmo-language is further-reduced
- But this set of restrictions expands the what the vanilla-flat-2018 search space is by more clearly defining the rule-space... I think?

...
- booleanization features... of 'tokenized' byte-data? (word-n-grams not byte n-grams)

1. flat level rules: rules for booleanizable relationships? (subest of regex?)
2. rules for output of one layer that allows it to be input to another layer...?

(breaking analogue data into tabular X columns?)... and comparing them against the original? e.g. n-gram A & n-gram B



(though there may be a multi-level-level too? rules for combining levels?)


TODO: check this:
```
"regular language" = set of strings accepted by a deterministic finite automaton. Restated as properties of a program that reads bytes:

- program state always with a pre-set finite range of options?
- program reads the input once, left to right, updating state on each byte

- updates use fixed function fn(state, "byte") → state 
(why 'byte')? not n-gram? or clause?)

```

Regular-ness is... part of this set of definitions but still too broad?

(Granmo-language is narrower than a regular language?)
- Regular
- Single Pass
- bounded? 
- Bitwise Operations/booleanized?



Q: layers of model being in alternate directions? (quasi-bi-directional?)


1. 'bounded-ness'
2. 'single linear pass'
3. translateable into linear bitwise operations
4. regular language definition: regular-ness
5. the overall iterative loop of retraining (unbounded) vs. specific sub-parts
6. finite-time computibility
7. 


Granmo tech Evolution note:
(The 2018 Granmo paper specifically said that it would not suggest any (and so not a tiered "threshold encoding"/bucket system) approach for the gray-scale MNIST study, deferring that topic for the future. It may or may not be relevant here what techniques were discussed later.)



--
### On the concept of a layered feature-discovering Granmo-Automata-Machine-System

Assumption: In the overall process of moving from raw data through a system of "models" (perhaps more generally, a 'compiler') to a final output (or final module output), there is no fundamental difference (there is only a semantic disciplinary-lexicon, aesthetics or  convention/custom difference) between pre-processing, encoding, feature engineering, feature discovery, feature pruning, or "representation"). While referred to in different ways it is bits or bytes in and bits or bytes out.  


## Layers:

Given that a classic 2018 Granmo-Automata-Machine-System performs classification, what 'classification' task would be performing the task of, (for example, going from raw ASCII (or unicode) bytes to the equivalent of N-Grams)?

A. Classification of one n-gram or byte type X 'feature' as useful or not
B. Classification of a set of n-gram or byte type X 'features' as useful or not

Question: To determine whether  the result of an earlier stage of selecting a type of feature selection is useful to the later model, is some form of 'back propagation' (if through whole process iteration) needed to provide a performance signal to the earlier 'more encoding/proprocessing oriented' layer?








Q1: Discontinuous Mono-Directional Bit/byte N-Grams?
Q2: Discontinuous Bi-Directional Bit/byte N-Grams?


Does this process only make sense in certain specific data-type and data-use situations, or can this more generally make sense for Granmo-Automata-Machine-Systems? Is this primarily a search-space problem?
How much is the issue of 'features' "not" being able to be evaluated in isolation an issue? (e.g. sets of features that work together, where no individual one by one search would find them?) 
Or, are there filters such as DF document frequency representation that may, if only for come use-cases or approaches, be able to prune that overly large search space?

..

Very side question:
Are there any approaches learned from chess-AI or other game space studies that might suggest methods of pruning search trees?






--

For the most part the lexicon around models and prediction in data science is not framed or oriented around writing functions, procedures, or code. In some cases should there be a more explicit focus of a goal of data science and automated learning being learning and producing a best-findable procedure-function, the best code, for a specified problem-space/use-case?

e.g.
A. A type/goal/scope of Granmo-Automata-Machine-System that outputs another Granmo-Automata-Machine-System

B. Either a Programming Language or a Compiler that uses something like a Granmo-Automata-Machine-System to compose and optimize functions/procedures/sub-routines for a specific task in a specific case.




..

# PCA, LDA, SVD, Granmo:
https://en.wikipedia.org/wiki/Linear_discriminant_analysis 
https://en.wikipedia.org/wiki/Principal_component_analysis 

PCA: Unsupervised
LDA: Fisher-type, Supervised


Q: PCA and Granmo-Automata-Machine-Systems

Two things stand out to me in my experience of the years with Data Science regarding PCA or principle component analysis: 

One: Perhaps echoing the decidedly ill-humoured inflammations around Fisher-Frequentist statistics and Bayesian statistics in the 1900s,
between finance oriented data science and bio-medical oriented data science where PCA is routinely used as a basic unsupervised model, yet the suggestion of the possibility of that provokes vitriol and rage among financial analysts who hurl conflagrations of abuse and defamation any anyone who violates the realist law that PCA is only for dimensionality reduction.

Two: The wonderful and highly recommended book by Hobson Lane "NLP In Action" (Manning press, https://www.manning.com/books/natural-language-processing-in-action the first edition I consider an eternal classic) thankfully goes into more detail than usual about how PCA is used, and often renamed or re-packaged, in many parts of many NLP modeling approaches (probably with many people never realizing that at a lower level the math is the same, the only difference being the Brand Sticker; leading inadvertently to reification-misunderstandings where people misconstrue the underlying mechanics: yet another case of communication and the psychological results of not following basic hygiene with naming: do not give one things two names, do not give two things one name). 

The point being: PCA is often fundimentally important in many ways, so how does PCA interface (or not) with the mechanics of Granmo-Automata-Machine-Systems?



Without getting too much in the weeds, the comparison of PCA/LDA and Granmo is poigniently mixed. On the one hand the stark difference in approach in terms of Automata Machine model training highlights how distant (and incompatible) AI/ML approaches can be while arguably outlining a diverse super-set of problem-spaces and processes where comparable problems can be comparably approached by incomparably different means, which is interesting. 

On the other hand both PCA and LDA could be useful tools for trying to narrow/prune larger (e.g. n-gram) feature sets.

- PCA-based Feature Selection 
- Subspace Leverage/Loading Analysis

Perhaps depending on the effectiveness of this 'hybrid-pipeline' approach, this may argue for more of a separation (if optional) of feature-processing vs. final training.

Results and data must be deferred to after various comparison tests, but it may be unlikely that a 'pure-vanilla-end-to-end-granmo-pipeline' to go from raw field-data bites through self data engineering to model training and operate in self-maintaining loops would be significantly more performant compared with an anything-goes-ensemble. Though there may be edge-cases, for example on extremely resource limited systems, where a Granmo-for-everything approach may be practical (or that may turn out to be usually more performant and practical). 

Ah...but if you suspected that this rabbit hole might get stranger, you might have been correct enough. As often happens with STEM Timelines, there has been increasing interest (e.g. since 2012 (image net), 2017 (attention is all you need) and 2022 (chatGPT)) in finding fasters ways to do linear-algebra matrix-maths (e.g. without the super-slow floating point operations that computer hardware really does not like); at the same time the research for this largely pre-dates the 'rush for optimization.'
 
In terms of keeping an eye on what is likely more practical and what is likely more speculative, part of what might come out of this is optimized ways to perform before-training operations (with the topic of making a hybrid bitwise Automata linear-algebra tensor system being... a bit of scope creep).

We can make three semi-related sets of research-leads, including one more related branch of inquiry. Each of these are whole areas with many papers and techniques, I will only list a few for brevity. 

1. Ways to Speed up & approximate floating point eigenvector operations:
e.g.
- Zou, H., Hastie, T. and Tibshirani, R. (2006) Sparse Principal Component Analysis. Journal of Computational and Graphical Statistics, 15, 265-286.


2. Ways to make a bitwise boolean version of linear algebra over a boolean matrix (e.g. to explore the boolean bitwise work of Granmo mixed with boolean bitwise linear algebra, if without an clear practical use)
e.g.
- "A Generalized Linear Model for Principal Component Analysis of Binary Data"
November 2002, Andrew Schein, Lawrence K. Saul, Lyle H. Ungar, University of Pennsylvania


3. Empirical Practical Tricks established by Game-Makers for very quickly and economically approximating slow expensive calculations.
e.g.
- https://github.com/othieno/GPBB ,Graphics Programming Black Book by Michael Abrash. http://en.wikipedia.org/wiki/Michael_Abrash 1997

- branchless, fixed-instruction Singular Value Decomposition, Aleka McAdams https://graphics.cs.wisc.edu/Papers/2011/MSTTS11/ 2011 


Note:
In the case of either before-training-feature-analysis/transformation or during training, this is not a situation where we 'need' to find a way to solve either every problem or any particular problem (e.g. a ship landing trajectory that must be solved). We only need to occasionally be able to find anything useful about anything (even a vague approximation), a much lower and much more empirical bar. In this use case, 'Greedy Approximation' is not a compromise or a tradeoff, it is a pure value optimization tool. Even a single useful find 1/100 looking at large pools of potential features, anything that works works.



..

Notes Along with the myth of symbols There are other historically, common assumptions that should be re-examined The primacy of hygiene As his perhaps well dramatized by the computer science. From 1930 to 2030. To pick somewhat arbitrary numbers. While early computer science was very much being developed at a time when paradigmatic models of extremism security issues and all that in the 1930s exemplify There has been an extremely stubborn recalcitrance Whereby Ivory Tower recreational mathematicians see the world is not extending beyond their professional recreational sandbox (as can be directly noted by the writings of the creators of C and the Internet decades after the empirical demonstrations of security issues from the 1960s Dartmouth Internet Not to mention the 1930s and 40s And rather noticed publications by writers, such as Norbert wiener (for example, the human use of human beings) Prolonged pathological refusal to acknowledge the area of cyber security, perhaps mirrors, the pathological refusal to accept the area of stem (for example, burning or imprisoning scientists, instead of creating a science department and letting them play in their sandbox) My own specialization if I have such a thing is in trying to articulate a more generalized notion of stem hygiene and so I am going to naturally propose that this is within the scope of any non-self-destructive organization


# Models in Functions that write code

How strange is it to be thinking about the use of Granmo-Automaton-Machine-Systems as a 'modeling' process in code that produces improved code? I would argue that while the semanitics of that are strange (and sounds like an Exotic Science-Fiction that people have been talking about for probably a long time (see: "AI Narratives" https://academic.oup.com/book/36637 ) but which is largely a naive 'realist-symbolist' fantasy) that this describes something rather mundane that is under our noses and largely ubiquitous: compilers. Though 'mundane' may be the wrong term, perhaps "apparently mundane" . The C-to-pdp-11 compiler was probably mundane-ish, but LLVM and the compiler debates around C, Zig, Odin (and I'm not sure about Rust aiming to separate from LLVM in 2026) are, I would argue, raising various questions strangely close to this paper's topic: auditable deterministic fast lite understandable optimization 'models.' While it is surely beyond the scope of this paper to try to propose a specific Granmo-LLVM-Compliler (or anti-LLVM), the relationship between GOFAI (and is there a better example of GOFAI than a fancy compiler?) or other machine learning and the production of other functions and code, or perhaps more simply looking at 'modeling' as native to coding and code architecture, should sound more familiar. It may be a sign of the evolution of programming into such extreme niche ideologies that the concept seems so alien. 
 

CppCon 2018: Matt Godbolt “The Bits Between the Bits: How We Get to main()”
https://www.youtube.com/watch?v=dOfucXtyEsU
