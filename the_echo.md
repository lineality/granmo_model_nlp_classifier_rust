
Echo-Revised
(very work in progress notes 2026 09 25 gga)

# Regular-Language Definition & Binarization for Granmo-Automata-Machines & Holzman C-Language Restricted Definition 

Note: Static-Analysis seems to be very consistent with the mode and approach of Granmo.

[TODO: singular plural in this sentence may be wrong 1-X, multiple X? 

The 2018 paper by Granmo describes a system that uses fixed length boolean X vectors, where x is an element of the set of all n-length vectors with elements in {0, 1} .
Granmo's 2018 paper refers to the processes/steps that produce a X-feature "Binarization," but does not describe a formal or defining constraint on the Binarization process (or range, or language, of processes).

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

Four of Holzmann's rules (for static-analysis verifiability of safety-critical C) enforce RL-Property-1,  RL-Property-2 &  RL-Property-3.


| Power-of-10 Rule | Automata-Theory Effect |
|---|---|
| No dynamic memory allocation after initialization | Program's state (entire) is fixed-size struct fixed at/before start ⇒(RL/RP-1), a finite-state set |
| All Loops statically provable to be 1. non-terminating, or 2. to have a constant upper bound | The Binarizer has one non-terminating loop (byte-read loop) that is the "read input left to right" of (RL/RP-2). Any loop in the read-loop is bounded so per-byte updates finish within fixed N steps ⇒ RL/RP-3 |
| No Recursion | No hidden stack growth or unbounded state ⇒ (RL/RP-1) |
| Pre-Allocated Buffers | Without dynamic memory, chunk-streaming must be used to process any non-fixed input. With streaming unbounded input, "regular" = "constant space." ⇒ (RL/RP-2) |

Static-analysis verifiability rules for C can be / are equivalent to Regular Language definition, such that a C-family procedure/function following these rules (with the byte-read loop as the one declared 'always on' non-terminating loop) is a Deterministic Finite Automaton (DFA), where a struct is a state set, a loop-body is the Binarizing procedure/function, and the read loop is the input "tape head" for an unbounded using a (DFA?) constant space.


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

Another factor to consider, which may or may not be rule-guided (if not rule defined per se), is factors/issues/metrics relating to computability. For example, there might be some kind of Big-O or 'computation load' context/measure/evaluation that we can add (which in a sense might, maybe, fit in with Holzman's bounding rule: if something is so big that it cannot fit in real-life memory, can that be defined as bounded?) This overall interdisciplinary system might even help to create a kind of computability-profile that might be especially important for resource-constrained environments (which, I speculate, will continue to exist).

Note: This may be an example of empirical vs. philosophical being both potentially useful or potentially distracting. Whether we are talking about 
Primarily we went to 'weigh' potential sets of features in order to determine (either with a useful hard or soft rule, or somehow) or inform the decision somehow of what model-parameters and feature-selection rules to use (empirically this will be a mix of entirely automated and some people wanted to do it by hand, but with data to go off/on).

There are potentially (at least) ~two ~ways that we can evaluate features and sets of features.
In terms of an individual feature, there is the question of whether every feature is computationally equal. If we can find ways that they are not, we may have a measure there. [In terms of individual features and] In terms of Big-O itself, since a regular feature is O(1) in L by definition, this is probably not what we are looking for.

For example, we may be able create per-feature cost metrics such as:
1. steps_per_byte_cost
2. bit_cost_of_feature or bits_of_state_stored_between_bytes
e.g.
- An arbitrary window W of bytes costs: 8w bits
- Finding a run of identical bytes ≥ r costs: ⌈log₂(r+1)⌉ bits + 8 bits  (TODO, check this)
- Getting a count of A saturating at k costs: ⌈log₂(k+1)⌉ bits (TODO, check this)
- Whether B exists within w-bytes after A costs: ⌈log₂(w+1)⌉ bits (TODO, check this)
etc.

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

Note: Some of this might apply more to the exotic-GA-derived features vs. the vanilla-extended-standard-features.


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

(TODO: check that this is a 'quadratic cost' as claimed by AI)
For example two Positional Features (from A-exists & B-exists, to A before B exists and B before A exists) has a Quadratic-cost, the potentially more useful N Positional Features (N-item positional tuples: A before B before C) is not trivial.


(Perhaps, as a rule of thumb, some Big'O states could be considered legal/regular, others illegal/irregular

#### Regular / Legal: (examples)
- Presence/absence of a token, byte, or n-gram (already in 2018-Granmo)
- Count (within whole "document") saturating at a constant ("seen at least k times")
- Recurrence within a fixed window of "document" ("Cat" mentioned more than once)
- A "run" of identical bytes of a fixed length ("five exclamation marks in a row")
- Position-relative within a fixed lookback (B after A)


#### Not Regular /  Illegal: (examples)
- Shannon entropy of byte distribution (needs 256 unbounded counters)
- compression ratio  (needs whole document)
- exact ratio of two unbounded counts (digit fraction, vowel fraction)
- TF-IDF as a real-valued score (needs an unbounded term count)


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
