treeR[1] = n;
treeR[n_] := treeR[n] = Table[o[treeR[a], treeR[n - a]], {a, 1, n - 1}]
treeC[n_] := Flatten[treeR[n] //. {o[a_List, b_] :> (o[#, b]& /@ a), o[a_, b_List] :> (o[a, #]& /@ b)}]
TreeForm[#, AspectRatio -> 0.618]& /@ treeC[4]
