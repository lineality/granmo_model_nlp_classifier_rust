Echo-Revised

(very work in progress notes 2026 09 25 gga)

# Regular-Language Definition & Binarization for Granmo-Automata-Machines & Holzman C-Language Restricted Definition 

Note: Static-Analysis seems to be very consistent with the mode and approach of Granmo.


The 2018 paper by Granmo describes a system that uses fixed length boolean X vectors, where x is an element of the set of all n-length vectors with elements in {0, 1} .
Granmo's 2018 paper refers to the processes/steps that produce an X-feature "Binarization," but does not describe a formal or defining constraint on the Binarization process (or range, or language, of processes).

It may be possible to describe this Binarization problem space using terms and conventions from a few areas/disciplines, including 'regular-language' systems, with very familiar examples such as RegeX.
For example: 
- A Binarizer can only recognize regular languages (probably a narrower sub-set of regular-language)
- A boolean-bit of (boolean)X must be decidable by a finite-state machine linearly processing input-data bytes.

The formal restrictions for Granmo Binarization may be identical to a set of Holzmann's "Power of 10" rules (2006, for restricting C)


## Regular Constraints

Let's look at what is or is not included in Regular-Language properties when looking at a Regular-Language as byte-strings suitable for a deterministic finite automaton: In terms of a byte-reading Regular-Program of functions/procedures: 

- RL Property 1: Regular-Program state is drawn from a finite set fixed before any input is seen (fixed at compile-time, pre-allocated), such that state does not grow with input.
- RL Property 2: Regular programs read input once ("left" to "right"), updating state after each byte
- RL Property 3: The update based on each byte is equivalent to a fixed function/procedure: fn/proc(state, byte) → state













## Holzmann's "Power of 10" & Automata Rules

Two disciplines (verifiable embedded code and formal language classes) may have authored the same machine specifications.

Three of Holzmann's rules (for static-analysis verifiability of safety-critical C) enforce RL-Property-1,  RL-Property-2 &  RL-Property-3.


| Power-of-10 Rule | Automata-Theory Effect |
|---|---|
| No dynamic memory allocation after initialization | Program's state (entire) is fixed-size struct fixed at/before start ⇒(RL/RP-1), a finite-state set; Without dynamic memory, chunk-streaming must be used to process any non-fixed input. With streaming unbounded input, "regular" = "constant space." ⇒ (RL/RP-2) |
| All Loops statically provable to be 1. non-terminating, or 2. to have a constant upper bound | The Binarizer has one non-terminating loop (byte-read loop) that is the "read input left to right" of (RL/RP-2). Any loop in the read-loop is bounded so per-byte updates finish within fixed N steps ⇒ RL/RP-3 |
| No Recursion | No hidden stack growth or unbounded state ⇒ (RL/RP-1) |

A set of Static-analysis verifiability rules for a programming language such as C can be equivalent to the definition of a Regular Language where document length is unbounded and document input is consumed as a chunk stream (never loaded whole) and when the program computes each bit exactly, regardless of document length, using a fixed-size state not dependent on some assumed maximum doc length. A C-family procedure/function following these rules (with the byte-read loop as the one declared 'always on' non-terminating loop) is a Deterministic Finite Automaton (DFA), where a struct is a state set, a loop-body is the Binarizing procedure/function, and the read loop is the input "tape head" for an unbounded using a (DFA?) constant space.


Theorem-shaped X,y statement -> A program that has a single non-terminating loop for byte-read, where all other inner loops are bounded, uses no recursion, uses no dynamic memory allocation (after initialization), processes unbounded input as a stream using a fixed-state, will compute exactly DFA-computable regular Binarizations.

Both directions work under the above conditions:

1. Program → DFA: Fixed memory of M bits leads to at most 2^M states, and each byte will trigger a deterministic update (DFA)

2. DFA → Program: A transition table with one read loop satisfies rules

Note: A Binarizer that is defined within these rules can process "documents" larger than that device's defined memory.

Machines with strictly constant space cannot always process unbounded input, for example if there is a need to store that entire input all at once. But a Deterministic Finite Automaton (DFA) can recognize any regular language in constant space; this is because the space complexity of a machine (like a Turing machine) is measured as a function of the input length n, and a DFA uses O(1) space. DFA memory (state(s)) is fixed and does not depend on changes in input size.

Also production-application oriented: A Binarizer process can run on a byte stream indefinitely, with document boundaries delimiter bytes in-band. Transition on delimiter bytes can emit X and reset state, so an outer loop is non-terminating (and Holzmann's proof obligation to "prove it cannot exit" is possible).

An arguably implicit requirement of Holzmann's rules is that any data being processed (such as a file or document or set of measurements, unlike a known to be fixed-in-size type value) if not already fixed in size must be able to be processed as (effectively) a stream of chunks: pre-allocating all memory precludes the horrible behavior of bulk loading and entire unknown-size input into (dynamic heap) memory. 

So we should specify:
(a) unbounded input is consumed as a chunk stream, never loaded in unknown whole size (which you cannot pre-allocate);
(b) the fixed-size state struct for unbounded input is modular and size is fixed.

To clarify, not all of Holzmann's Rules relate to language definition, or at least are not pursued and discussed here.


## Defining A Regular Legality

Since there is a forever-goal of finding new ways to increase the possible set of useful and use-able feature sets and types (and any other tricks or treats), something constructive that might emerge from this witches caldron is a more understandable and interdisciplinary way to describe (to define) a delineation between legal and illegal types of X-features, by being able to systematically evaluate the effect that computing a feature would have on state. This may give three general buckets of features:

1. "Yes": (it might not help the model-performance, but you can go ahead the test it)
2. "Not as such": you need a trick: and hopefully the rules will help to guide exactly where the trick needs to be (one-directional, bounded, etc.)
3. "No": (Though some people might see this as a challenge where if you look hard enough there is always some kind of trick.)

Another factor to consider, which may or may not be rule-guided (if not rule defined per se), is factors/issues/metrics relating to computability. For example, there might be some kind of Big-O or 'computation load' context/measure/evaluation that we can add (which in a sense might, maybe, fit in with Holzmann's bounding rule: if something is so big that it cannot fit in real-life memory, can that be defined as bounded?) This overall interdisciplinary system might even help to create a kind of computability-profile that might be especially important for resource-constrained environments (which, I speculate, will continue to exist).

Note: This may be an example of empirical vs. philosophical being both potentially useful or potentially distracting. Whether we are talking about 
Primarily we went to 'weigh' potential sets of features in order to determine (either with a useful hard or soft rule, or somehow) or inform the decision somehow of what model-parameters and feature-selection rules to use (empirically this will be a mix of entirely automated and some people wanted to do it by hand, but with data to go off/on).

There are potentially (at least) ~two ~ways that we can evaluate features and sets of features.
In terms of an individual feature, there is the question of whether every feature is computationally equal. If we can find ways that they are not, we may have a measure there. [In terms of individual features and] In terms of Big-O itself, since a regular feature is O(1) in L by definition, this is probably not what we are looking for.

For example, we may be able create per-feature cost metrics such as:
1. steps_per_byte_cost
2. bit_cost_of_feature or bits_of_state_stored_between_bytes
e.g.
- An arbitrary window W of bytes costs: 8w bits
- Finding a run of identical bytes ≥ r costs: ⌈log₂(r+1)⌉ bits + 8 bits
- Getting a count of A saturating at k costs: ⌈log₂(k+1)⌉ bits
- Whether B exists within w-bytes after A costs: ⌈log₂(w+1)⌉ bits
- etc.

So we might have four measures:
1. steps_per_byte_cost
2. bit_cost_of_feature
3. number_of_features

Where, if not simplistically (or not always simplistically) there could be model parameters to specify a max-budget for each of these, such that feature-sets or features can be included or excluded. 

As this is simplistic, there could also be a top_df or top_mutual_information (or some other criteria or mix of criteria) where a feature is only applied to that top-slice of vastly more possible byte-grams.

E.g.
bit_cost_buget = ?
steps_per_byte_budget = ?
feature_quantity_budget = ?
top_filter_threshold = 10

Note: Some of this might apply more to the exotic-Genetic Algorithm-derived features vs. the vanilla-extended-standard-features.


So a different set of categories (for example where there is always a direct-yes or a trick-yes) might look like this, with two boolean properties:
1. type_is_legal: true/false
2. cost_is_in_budget: true/false

As a list of options (buckets):
1. Direct-Yes & Slim-Enough
2. Direct-Yes but Too Heavy
3. Proxy-Trick-Yes & Slim-Enough
4. Proxy-Trick-Yes but Too-Heavy
5. No Proxy Found / Lack of Imagination (to cover all situations)

And where proxy-trick vs. direct-yes is not important enough to record, the only factor may end up being state_size, or the potential/empirical abundance of a given feature.
```
cost_is_in_budget: true/false
```

For example two Positional Features (from A-exists & B-exists, to A before B exists and B before A exists) has a Quadratic-cost, the potentially more useful N Positional Features (N-item positional tuples: A before B before C) is not trivial.

#### Positional-features can have a quadratic cost:

This has a quadratic cost In candidate-feature count over vocabulary size (`number_of_features`), not in per-feature `bit_cost_of_feature` or `steps_per_byte_cost` in inference, both of which stay at O(1) regardless of size.

Generating an A-before-B / B-before-A feature for every ordered pair in an N-item vocabulary (V) produces V² candidates; generating every ordered k-tuple produces V^k candidates. (The "not trivial" growth mentioned above.) 

Question: k-tuple counts (V^k vs. P(V,k)

For any two items A and B there are only 2 features. As to which process is being referred to, the (oracle/build-process(before inference) or inference-process, this cost is a candidate-generation/pruning cost, not a property of the feature type itself. I.e. the budget for `number_of_features` with or without some pre-filter-criteria such as `top_df` or `top_mutual_information,` as mentioned above.

#### On Feature-Count: for budgeting `number_of_features` with vocabulary size V
- Fixed-length n-gram (length exactly n): has V^n possible features
- n-grams over length range [n_min, n_max]: Σ (n=n_min to n_max) V^n
- Ordered k-tuple of distinct items: P(V,k) = V!/(V−k)!; (≈ V^k only as a rough upper-bound for V ≫ k, quick-estimate/proxy is worse for bigger k or a smaller V
- Ordered k-tuple allows repeated items (e.g. "A, then A again, then B"): V^k (Overlaps somewhat with "saturating count")

(Perhaps, as a rule of thumb, some Big'O states could be considered legal/regular, others illegal/irregular

#### Regular / Legal: (examples)
- Presence/absence of a token, byte, or n-gram (already in 2018-Granmo)
- Count (within whole "document") saturating at a constant ("seen at least k times, e.g. "Cat" mentioned more than once in the whole doc")
- Recurrence within a fixed window of "document" ('Cat' appears twice within e.g. 100 bytes.")
- A "run" of identical bytes of a fixed length ("five exclamation marks in a row")
- Position-relative within a fixed lookback (B after A); cost is ⌈log₂(w+1)⌉ bits (tracks elapsed bytes)
- A anywhere before B: (order only, no distance bound (cheaper)): 2 bits of state; 3 states: {seen neither / seen A / seen A-then-B (sticky)} → ⌈log₂3⌉ = 2
- Ordered k-tuple: "A before B before C …" (k specific items; in order; distances unbounded, uncounted): ⌈log₂(k+1)⌉ bits of state (milestone counter, 0..k)
(Notes: Ordered k-tuple is not the same as ~'B after A within a fixed lookback' to track elapsed bytes at cost ⌈log₂(w+1)⌉ bits (it). Orderd-k-tuples should be cheaper because they are not bound and do not count distance. The milestone counter only tracks which of the k items is sought next. If the items are multi-byte words rather than single bytes, then a cursor is needed to track partial-progress with sufficient bits to count the length of a longest word.)

#### Not Regular /  Illegal: (examples)
- Shannon entropy of byte distribution (needs 256 unbounded counters)
- compression ratio  (sometimes needs whole document)
- exact ratio of two unbounded counts (digit fraction, vowel fraction)
- TF-IDF as a real-valued score (needs an unbounded term count)

Note:  gzip/DEFLATE makes use of a bounded window: 32 KB. This compression ratio stays illegal because it is a bounded ratio form reflecting two unbounded counts (compressed length / uncompressed length). This case is similar to digit fractions.

Note: The TF-IDF proxy count(X) ≥ ⌈θ / IDF(X)⌉ is valid if TF is a raw count, but If TF is normalized (by document length) then as a ratio it instead needs length-bucket × threshold grid.

### Regular-Proxies, Tricks & Treats
Illegal features should (perhaps all) have a regular proxy, and perhaps all of those can be single-operations such as capping a counter (using a constant) or  and replacing ratios with a grid/series of length-bucket and threshold proxies "thermometer encoding."


| Illegal / Irregular Literal Feature | Regular Proxy |
|---|---|
| Shannon entropy | "≥ k distinct byte values seen" (k fixed); "maximum run of identical bytes ≥ r" |
| compression ratio | "n-gram recurs in window w"; "run-length ≥ r" |
| digit fraction > 50% | `length ∈ bucket_b ∧ digit_count ≥ k_b`, one test per bucket |
| TF-IDF(X) ≥ θ | `count(X) ≥ ⌈θ / IDF(X)⌉`, IDF(X)  as a constant |

Note: In a 2025 paper Granmo et al process grey-scale MNIST using "thermometer encoding." There may be a not-publically-available Granmo-paper prior to 2025, but I cannot locate any such paper (publically available, or as described in a publically available abstract). Alongside the 2018 paper specifically deferring the grey-scale topic, these two papers exist:
- "An All-digital 65-nm Tsetlin Machine Image Classification Accelerator with 8.6 nJ per MNIST Frame at 60.3k Frames per Second" Jan 2025, by Svein Anders Tunheim, Ole-Christoffer Granmo, et al
https://arxiv.org/html/2501.19347v1 
- J. Buckman, A. Roy, C. Raffel, and I. Goodfellow, “Thermometer encoding: One hot way to resist adversarial examples,” in ICLR, 2018.

A possibly related paper is https://arxiv.org/abs/1905.04199v2 arXiv:1905.04199v2 [cs.LG] 21 Jun 2019
"A Scheme for Continuous Input to the Tsetlin Machine with Applications to Forecasting Disease Outbreaks" K. Darshana Abeyrathna Ole-Christoffer Granmo Xuan Zhang Morten Goodwin Centre for Artificial Intelligence Research, University of Agder, Grimstad, Norway, but this paper does not use the term "thermometer", refer to MNIST, refer to "gray scale," or any "scale." It does refer to using "thresholds" of continuous values to derive binary features.

## Modules, Layers, Oracles: Evaporating Oracles vs. Ideological Homogeneity
As a note on layers, so far as I understand: the Granmo-automata-machine-module under discussion is itself flat. The definitions that we are concerned with here, as I understand it, are regarding only a single flat module. How those modules are arranged with various other modules and diverse parts are not the topic of these definitions. Questions about what can be built using these module-level definitions is arguably relevant, but the definitions themselves are focused on the module-level.

One interesting set of questions, which may have different answers depending on very different real-life use cases, is to what degree there can be or should be a non-granmo-module oracle in a pipeline, and the maybe or maybe not twisty question of whether you could officially put a Granmo-module inside another: for example, would having a nested set of gram-models continually processing data violate a rule against recursion... is that a silly question or might it not be a silly question? 

(It may be entirely absurd to try or ask what would happen empirically or definitionally if you created a doughnut loop of GAMS, but it something useful might come from that engine component property set)

If having modules be not inside each other but arrangable in 'time and space' avoids that questions either way, and with a series of Granmo models operating in cycles with empirical data but without non-granmo modules, and where Granmo models use the empirical data and proxy tricks to adapt, could that push the role of oracle onto either the empirical source of data itself (probably often if marginally affected by the measuring and data processing in some way) or onto a configuration but not a process of 'feature' of the system... or not.

There may not be a use-case where this is relevant, but the old Ant/swarm scenario might be an example. Within a normal posix-server, how you set up your pipeline is not usually relevant, such that there is no advantage or imperative to have one model be like or unlike another. But in a module-swarm decentralized data-processing situation, there may be various factors whereby trying to introduce non-module kludges might be a liability. 



the path of least resistance may be to keep the 'language' theme, and to refer to 'tokenizing' as an instrumentalist subset of the continuum of data processing from input to output
that we may be able to define into the Granmo-Language phase and out of the oracle feature-set-design phase


# Recursion:
For planning purposes, let's go back (at a high level) to everyone's favorite topic: recursion and computation. 

1. Godel 1931
2. Turing-Church 1936
3. Turing Oracles & NP-Completeness 1938

4. Recursion in dynamical, chaotic, and nonlinear systems
5. Holzmann 2006, removing recursion from statically analyzable behavior-stable systems and for stack management: stack-recursion growth (possibly also the general incomprehensibility of the tangles for critical-code)


1. Logical Godel Recursion: ~causes static-analysis issues but no a stack-overflow issue (non-stack?), issues with provability (Q: provability vs. static analysis? vs. memory-behavior?)
2. Stack-Recursion: where stack-growth is an issue
3. Dynamical Feedback Recursion: nonlinear and unpredictable behavior (etc.)

Is it so simple as to say: creating a feedback loop of granmo-machines isn't recursion because there is no stack overflow... I am not so sure the overall picture is that simple. No stack-overflow, great! But the larger story of recursion and computation should not be easily shoved into the skeleton filled closet under the stairs. 


6: Looped granmo-models are not stack-recursion... 


Non-Absolute modeling & a "proto-type cmpiler"

Possibly one other way to slice into this Lasagna is that some recursion, godel, NP-complete and oracle issue have to do (maybe) with absolute proven 'models' vs. approximate-sufficient models (maybe relating to 'statistical' but sub-symbolic models are unclearly related to other branches of 'statistics' (though I would argue that the hyphenated marriage is unavoidable). 

This might get into the next rabbit hole of this investigation (compilers and memory-behavior-modeling), but in the realm of computation the landscape is more empirical and finding sufficient non-absolute (perhaps lossy) approximations is the name of the game (not absolute pure provable equality). This may be more pronounced in data-science where indirect pursuit of a 'general' pattern (that should not exactly (over) fit the existing data) is explicit.

For example, when the topic of modeling becomes 'optimizing' operations (for a specific criteria) for performance or size of binary etc., there is likely a significant pivot from pure absolute brute force logical proof to as barely as possible sufficient thrifty-resource-economic means, though strictly 'modeling' memory behavior for safe-behavior may like be more of the absolute variety whereas modeling the operations to be economic within safety bounds may be more in the fuzzy-general-bias statistical direction.

While it may be impractical to ask about using gram-machines and these computation-formalisms when making a ~compiler (checker-optimizer) for assembler with no language abstractions or features on top of it, it may be an interesting test-case for how the historically estranged threads of these topics may act when brought back together. 

- safe-behavior memory models
- viable optimization models
- effective task equivalence models
- 


# Yet Another Reiterated Clarification on Regularity Claims

The overall goal and context here is to empirically expand the range of binary features that a Granmo Automata Machine-System can make and use, presumably with inference being separately and probably more narrowly defined than feature-discovery (or searching, engineering, vetting, comparing, testing, and deciding on what features to use). In this context I think it is relevant to look at related definitions that may be useful, such as definitions around regular expressions and definitions around statically-analyzable operations. I am repelled by arbitrary theory-of-everything fetishising and pedantic trolling that is irrelevant for useful testable applications. No one pattern is all patterns. There is no point in looking for or arguing over some simplistic principle that is hoped to somehow govern every possible use of every possible "model" in every possible way in every possible context; that is painfully meaningless and useless. Addicts should seek medical help, or at least not derail real world projects. Where useful I advocate for looking at the properties of regular languages, but not beyond that. 

With that caveat, hopefully the following discussion stays in the context of what is practical and relevant for concretely expanding the scope of features used in flat Granmo inference.

## Where the definition applies: 1. Regular Binarizer, 2. Per-bit definition

####  1. Regular Binarizer
Proposed features are evaluated for legality before the model is designed and trained. In order to be a legal feature, it must be the case that per-document computation for this feature can be calculated at inference time with fixed-state and be valid for all documents regardless of length. 

The regularity rule is enforced only on the Binarizer because that is the only place where there could be a breaking-violation. The Binarizer is the stage where raw input (the input "document") has unbounded length, and so it is the only stage where state could potentially grow with input length (which is not allowed to happen). The other downstream stages (clause ANDs over x; vote sum over clause outputs; threshold producing a single output bit) operate on fixed-size inputs and are more inherently finite-state by construction. 


### 2. Per-bit definition
For each bit 'i' of x, we can define a language L sub i = { documents d : x sub i (d) = 1 }, and define a legal binarizer as being that every L sub i is a regular language.

A per-bit definition may be equivalent to a whole-Binarizer definition. If each L sub i is decided by a finite automaton, then all such decisions can run in one pass. The result of finitely many finite-automata is also a finite-automaton.

The practical outcome of per-bit definition is that features can be evaluated (including budgeting and pruning) independently. This supports both feature-search, potentially including GA feature experimentation, and bit_cost budget analysis.


## Max/Capped Input Length

If indirectly, there is an echo of the topic of input length or input modularity in Holzmann in his (paraphrased here) suggestion that functions, which may be thought of as (my paraphrasing) modules or files or input files, have a (my paraphrasing) hard length cap. There is of course no literal separate file per function per module length cap argued to be required for static analysis in such rigid terms, but the topic is clearly given both mention and discussion real estate in his no-frills paper. And this may if more indirectly relate to questions around pre-allocated vs. dynamic memory (where processed modules have a capped size). There are various related topics here, from work-place logistics to long-term maintainability to security concerns to formal definitions. Hopefully 'echo' is a middle ground here. 

The point/issue/topic of capping/fixing or leaving unbounded input length (for inputs to be modeled in this case, for modules of a system to be static-analyzed in other cases) may seem somewhat pedantic, but it may (or may not) be an important point in terms of defining systems, for example systems with different categories and types of uses and use-contexts. Systems with size-capped inputs might have significant definitional differences compared with systems designed to (not to say are "required" to) accommodate dynamic whole input loading of unknown sizes. (And this might be a micro-cosm of a larger problem in computer science of input overflow and size and type enforcement. This may relate to a larger or separate or subset(?) topic of cases/contexts or systems where processing either can or cannot be 'bucket-brigade' managed in either a capped or arbitrary number of modular fixed-state sub-processes (perhaps also related to 'streaming').)

There are a number of dichotomous distinctions in Data Science, but I am not familiar with distinctions (such as basic model types and definitions) around fixed, caped, or unbounded input lengths. That is not normally a key topic when talking about regression, trees, logistic models, clustering, artificial neural networks, etc. 

As we bring together more formal discussions of computation, modeling, languages, and formal systems, this distinction about required, anticipated, or artificially enforced input lengths may be important to dwell on sufficiently. With or without reference to computability, completeness, language-type, etc. some models may take the shortcut of imposing a fixed input length (whether or not that is to achieve a formal rule requirement). In many cases this is probably a justifiable optimization, but it should at least be noted for zooming out to look at various modeling cases including NLP, time-series, tabular-one-hot, etc. (E.g. for a purely tabular one-hot data input (or reasonably bounded input such as 0-1 flat, percent, or effectively fixed range measurements (temperature measures are not going to be infinite or an unknown range the kelvin metric (or for exotic physics is that maybe not safe to assume?), in these cases arguably input-length is not relevant for that type of model. But what does this tell us about the overall landscape and problem space of models and their properties? 

What happens if assumptions about input length are or are not made? (In the practical history of computer science, typically assumptions are casually and confidently made ("Let's just say it won't be bigger than this and move on quickly.") with disastrous apparently-unforeseen future consequences.) 



In our case, where language definition, regularness definition, binarization, and phases of model-design feature-engineering and inference-only stages/phases are being defined, input length seems to be in the foreground.

We have probably a concrete note here regarding requirements for testing features as part of a feature-workshop-engineering-testing-vetting process to make sure they fit into the Granmo-Language filter so they they will behave properly as features; and we may have another foot in the door point for looking at how approaches to modeling and should relate to other areas of computer science, information theory, STEM, etc. 

For feature vetting: How potential/legal/valid granmo-features (evaluated with regards to final inference but vetted/tested in a previous ~'oracle-type' stage) are evaluated; specifically, pointing out potential erroneous steps, assumptions, practices, or processes that would invalidate the validation-outcome.


1. is_legal: Testing unbounded-input-length (for feature-vetting/validation)
A feature validity/legality test needs to be framed/defined in a context of an unbounded input handled as a stream of chunks (not as a fixed-length, bounded, truncated, or otherwise capped data-input max length). A. Capping the document length would interfere with evaluating the computational-behavior properties of that feature, for example artificially making even known to be invalid features such as shannon-entropy appear valid (a technicality of fixed-length and regularness). B. In the speculative and background-understanding realm, this might provide interesting ways to look at assumptions, boundaries, and behaviors around many common statistical-learning AI/ML models that assume or require a fixed-size input (what does that obscure or impose or truncate about the behavior and use of the model, and what edge-cases and error-spaces are exposed?)

2. fits_budget: Testing memory-growth with "Doc"-length (for feature-vetting/validation)
Test 2: Feature must be zero-memory-growth as "doc" input-length grows. 

Does, and how does, memory-use grow with increasing "document" (input data) length?

It is not sufficient to say that the program itself has finite state, because 'being finite' does not describe generally or specifically how memory is used. For example, a fact of using a bounded N-bit integer is not sufficient if a counter stored in that integer would roll-over beyond a given size; this would not cause the 'machine' to crash, or preclude it from compiling or running, but it means that silent failure or a silent-error (that is physically computationally legal) produces an invalid "overflow" (if that term is permissible here) value error (e.g. a 0-63 count of 64 becoming a count of zero). 

Part of the memory-growth-test is how the goal of the feature is expressed, namely a non-growing proxy (fine) vs. a growing raw feature (not fine). There is probably some case-by-case theoretical wiggle-room where a very slowly growing feature is tolerable given a preponderance of evidence about input-size, but that may be like using an unsafe code block in Rust; the default rule should be: The feature must be zero growth, if the candidate feature does grows then find/make a proxy that does not grow and test that (most growing-features should have non-growing proxies). Understanding how memory-use grows may help to design (or automate design) of a useful proxy.
