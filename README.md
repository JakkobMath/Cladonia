# Cladonia
Should eventually be a vaguely ok chess engine. 

Cladonia makes an effort to be as extensible as possible by implementing functions polymorphically (even when it's definitely unnecessary to do so) and dividing up the code into compact modules. It will contain at least one game module and an engine module. Game modules consist of 1) an "abstracts" module following a "no magic types" principle that describes the rules of the game at the trait level (movegen, legality checking, end-of-game detection) and 2) at least one "implementations" module that implements those traits (probably overriding functions for performance reasons) for particular types. I haven't written the engine module yet, but the plan is for it to contain search code that again remains as agnostic as possible as to which types it's dealing with (but allows for game-specific optimizations provided to it by the game module and becomes more sophisticated when given evaluators with additional properties such as admitting windows for AB search or containing instructions to use transposition tables/ endgame tablebases/ opening books). Eventually game modules may also contain game-specific protocol information such as the UCI specification. 

This is probably not the best way to make a game engine, but it is the kind of puzzle I enjoy solving and it should allow me to do some pretty experimental things (look forward to some really weird representations of positions' values, new bad ideas with "killer move" vibes, and more)!

Feedback is welcome. 
