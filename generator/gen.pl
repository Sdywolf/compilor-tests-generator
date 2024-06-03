:- use_module(library(clpz)).
:- use_module(library(lists)).
:- use_module(library(iso_ext)).
:- use_module(library(debug)).
:- use_module(library(builtins)).
:- use_module(library(freeze)).
:- use_module(library(format)).

in(X, Min, Max) :-
    X #>= Min, X #=< Max.

node(n(Id, K, use), K, Id).
node(n(Id, K, def), K, Id).
node(n(Id, 0, block), 0, Id).
node(n(Id, 0, loop), 0, Id).
node(n(Id, 0, brk), 0, Id).

tree(nil, 0, Ks, Ks, Ids, Ids).
tree(t(V, L, R), N, [K| Ks], NKs, [Id| Ids], NIds) :-
    node(V, K, Id),
    N1 #>= 0,
    N2 #>= 0, 
    N #= N1 + N2 + 1,
    tree(L, N1, Ks, TKs, Ids, TIds),
    tree(R, N2, TKs, NKs, TIds, NIds).

vis(T,T).
vis(T, t(_, L, _)) :-
    vis(T, L).
vis(T, t(_, _, R)) :-
    vis(T, R).

inL(T1, t(_, T3, _)) :-
    vis(T1, T3).

inR(T1, t(_, _, T3)) :-
    vis(T1, T3).

foreach(Generator, Goal) :-
    findall(Goal, Generator, Goals),
    maplist(call, Goals).

varlist(0,[]).
varlist(N,[_|L]) :- N#>0, M#=N-1, varlist(M,L).

fd_domain([], _, _).
fd_domain([N| Ns], Min, Max) :-
    N in Min..Max,
    fd_domain(Ns, Min, Max).

def_use(T) :-
    foreach((vis(S,T), S = t(n(_, K, use), _, _)), 
        (vis(R,T), R = t(n(_, K, def), _, _), vis(S, R))).

brk_in_loop(T) :-
    foreach((vis(S,T), S = t(n(_, _, brk), _, _)), 
        (vis(R,T), R = t(n(_, _, loop), _, _), inL(S, R))).

well_form(nil).
well_form(t(n(_, _, use), L, R)) :-
    L = nil,
    well_form(R).
well_form(t(n(_, _, def), L, R)) :-
    L = nil, 
    well_form(R).
well_form(t(n(_, 0, brk), L, R)) :-
    L = nil, 
    well_form(R).
well_form(t(n(_, 0, block), L, R)) :-
    well_form(L), well_form(R).
well_form(t(n(_, 0, loop), L, R)) :-
    well_form(L), well_form(R).

real_well_form(t(n(_, 0, block), L, R)) :-
    well_form(L), well_form(R).

is_subtree(T, T).
is_subtree(T, t(_, L, R)) :-
    is_subtree(T, L); is_subtree(T, R).

isnt_subtree(T, T) :- false.
isnt_subtree(_, nil) :- true.
isnt_subtree(T, t(_, L, R)) :-
    isnt_subtree(T, L), isnt_subtree(T, R).

increasing([]).
increasing([H1|[H2]]) :-
    H1 #< H2.
increasing([H1|[H2|T]]) :-
    H1 #< H2, increasing([H2|T]).

generator(T, N) :-
    varlist(N, Keys), varlist(N, Ids), 
    increasing(Ids), 
    Max #= N - 1, 
    fd_domain(Keys, 0, Max), fd_domain(Ids, 0, Max), 
    label(Ids), 
    tree(T, N, Keys, [], Ids, []),
    real_well_form(T),
    brk_in_loop(T),
    label(Keys), 
    def_use(T).

% def_use(nil, _).
% def_use(t(n(X, use), _, _), Xs) :-
%     member(X, Xs).
% def_use(t(n(X, def), L, R), Xs) :-
%     def_use(L, [X| Xs]), def_use(R, [X| Xs]).
