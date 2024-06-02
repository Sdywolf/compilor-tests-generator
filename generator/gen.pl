:- use_module(library(clpz)).
:- use_module(library(lists)).
:- use_module(library(iso_ext)).
:- use_module(library(debug)).
:- use_module(library(builtins)).
:- use_module(library(freeze)).
:- use_module(library(format)).

in(X, Min, Max) :-
    X #>= Min, X #=< Max.

node(n(K, use), K).
node(n(K, def), K).
node(n(0, block), 0).

tree(nil, 0, Ks, Ks).
tree(t(V, L, R), N, [K| Ks], NKs) :-
    node(V, K),
    N1 #>= 0,
    N2 #>= 0, 
    N #= N1 + N2 + 1,
    tree(L, N1, Ks, TKs),
    tree(R, N2, TKs, NKs).

vis(T,T).
vis(T1, t(_, T3, _)) :-
    vis(T1, T3).
vis(T1, t(_, _, T3)) :-
    vis(T1, T3).

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
    foreach((vis(S,T), =(S, t(n(K, use), _, _))), 
        (vis(R,T), =(R, t(n(K, def), _, _)), vis(S, R))).

well_form(nil).
well_form(t(n(_, use), L, R)) :-
    L = nil,
    well_form(R).
well_form(t(n(_, def), L, R)) :-
    L = nil, 
    well_form(R).
well_form(t(n(_, block), L, R)) :-
    well_form(L), well_form(R).

real_well_form(t(n(0, block), L, R)) :-
    well_form(L), well_form(R).

is_subtree(T, T).
is_subtree(T, t(_, L, R)) :-
    is_subtree(T, L); is_subtree(T, R).

generator(T, N) :-
    varlist(N, Keys),
    Max #= N - 1, 
    fd_domain(Keys, 0, Max), 
    tree(T, N, Keys, []),
    real_well_form(T),
    label(Keys), 
    def_use(T).

% def_use(nil, _).
% def_use(t(n(X, use), _, _), Xs) :-
%     member(X, Xs).
% def_use(t(n(X, def), L, R), Xs) :-
%     def_use(L, [X| Xs]), def_use(R, [X| Xs]).
