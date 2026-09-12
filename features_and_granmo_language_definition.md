features_and_granmo_language_definition

# Feature Discovery & Language Definitions in Granmo Models
2026 09 10th Geoffrey Gordon Ashbrook


One of the underlying, or over-arching, themes in this set of experiments and explorations, being in some respects narrow (if overly) and in other respects broad (if overly), is the topic of feature discovery and the agenda of expanding a possible, coherent, 'collaborating' set or family (or language) of features for one (narrow) problem-space or more (broad) problems more generally.

Narrow:
Starting from an NLP (natural language process) context, we will look at trying to make two additions, or two levels of additions to a 2018 'vanilla' Granmo Model (TM):


## Level 1: The Standard Library
Can extend the standard default feature space used by 2018-Granmo Models? This question is largely empirical.

Is this, or can we define this, extended standard feature set as an implicitly native possible feature set that was nascent already in the 2018-Granmo system definition, where the 2018-Granmo definition was (this is the question to investigate) more minimal than necessary? This question is more abstract but should feed into other followup empirical experiments.

The starting point for level-one (again, from an NLP perspective) is looking for a subset of RegeX operations that can be translated into 2018-Granmo-Model DFN bitwise-operation compatible features.

The question will then be framed as a language-definition question: Something like: can Granmo-model feature space be described as a formal language that is a super-set of the original 2018 feature-scope (and is this practical for any use-cases to improve model reach)?



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


Feature Pre-Analysis:

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
