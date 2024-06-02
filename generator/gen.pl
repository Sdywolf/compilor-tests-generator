:- use_module(library(clpz)).
:- use_module(library(lists)).
:- use_module(library(iso_ext)).
:- use_module(library(debug)).
:- use_module(library(builtins)).
:- use_module(library(freeze)).

n_factorial0(0, 1).
n_factorial0(N, F) :-
    N #> 0,
    N1 #= N - 1,
    n_factorial0(N1, F1),
    F #= N * F1.

n_factorial(0, 1).
n_factorial(N, F) :-
    N #> 0,
    N1 #= N - 1,
    F #= N * F1,
    n_factorial(N1, F1).

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

def_use(T, N) :-
    varlist(N, Keys),
    Max #= N - 1, 
    fd_domain(Keys, 0, Max), 
    tree(T, N, Keys, []),
    label(Keys), 
    foreach((vis(S,T), =(S, t(n(K, use), _, _))), 
        (vis(R,T), =(R, t(n(K, def), _, _)), vis(S, R))).

% def_use(nil, _).
% def_use(t(n(X, use), _, _), Xs) :-
%     member(X, Xs).
% def_use(t(n(X, def), L, R), Xs) :-
%     def_use(L, [X| Xs]), def_use(R, [X| Xs]).
