

Parameter interconnection:
(speculative)
These parameters may need to be raised/lowered together:


--vocab-size = "M" in Granmo terms.

--epochs
"Each clause must evaluate and calibrate 2M automata (positive and negated twins). Tail n -grams appear in far fewer documents, requiring more passes across the corpus for their automata to see enough updates to reach stable include/exclude decisions."

--specificity
"With more features, false literals appear more frequently; if s  is too low, clauses decay too aggressively, but if s is too high, clauses hoard spurious negated literals for absent rare tokens."
 
--states
"As M grows, the tail consists of rare n -grams that appear only a handful of times. If N is too low, rare spuriously correlated tokens can push automata past the N boundary (false inclusion). A sufficient N acts as an inertial noise filter against tail tokens."
