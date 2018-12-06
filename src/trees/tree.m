(* ::Package:: *)

expr2str[expr_] := StringTake[
    StringReplace[
        ToString@FullForm[expr],
        {
            "Slot" -> "box Number",
            "Plus" -> "box Plus",
            "Subtract" -> "box Minus",
            "Times" -> "box Times",
            "Divide" -> "box Divide",
            "Power" -> "box Power",
            "Surd" -> "box Surd",
            "[" -> "(",
            "]" -> ")",
            "1" -> "a1",
            "2" -> "a2",
            "3" -> "a3",
            "4" -> "a4"
        }
    ],
    {14, -2}
];

o2[n_] := Block[
    {s = Slot /@ Range[n], gps1, gps2},
    gps1 = Groupings[Permutations[s, {n}], {Plus -> {2, Orderless}, Subtract -> 2}, HoldForm];
    gps2 = DeleteDuplicatesBy[gps1, Factor@*ReleaseHold];
    TemplateApply[
        "pub const V`4`O2:&[fn(`1`) -> AST<Maybe32>; `2`] = &[`3`];",
        {
            StringRiffle[ConstantArray["Maybe32", n], ","],
            Length@gps2,
            StringRiffle[StringReplace[ToString[s], {"#" -> "a", "{" -> "|", "}" -> "|"}] <> expr2str[#]& /@ gps2, ","],
            n
        }
    ]
];

o4[n_] := Block[
    {s = Slot /@ Range[n], gps1, gps2},
    gps1 = Groupings[Permutations[s, {n}], {Plus -> {2, Orderless}, Subtract -> 2, Times -> {2, Orderless}, Divide -> 2}, HoldForm];
    gps2 = DeleteDuplicatesBy[gps1, Factor@*ReleaseHold];
    TemplateApply[
        "pub const V`4`O4:&[\nfn(`1`) -> AST<Maybe32>; `2`] = &[\n`3`\n];",
        {
            StringRiffle[ConstantArray["Maybe32", n], ","],
            Length@gps2,
            StringRiffle[StringReplace[ToString[s], {"#" -> "a", "{" -> "|", "}" -> "|"}] <> expr2str[#]& /@ gps2, ",\n"],
            n
        }
    ]
];
o6[n_] := Block[
    {s = Slot /@ Range[n], gps1, gps2},
    gps1 = Groupings[Permutations[s, {n}], {Plus -> {2, Orderless}, Subtract -> 2, Times -> {2, Orderless}, Divide -> 2, Power -> 2, Surd -> 2}, HoldForm];
    gps2 = DeleteDuplicatesBy[gps1, Factor@*ReleaseHold];
    TemplateApply[
        "pub const V`4`O6:&[fn(`1`) -> AST<Maybe32>; `2`] = &[`3`];",
        {
            StringRiffle[ConstantArray["Maybe32", n], ","],
            Length@gps2,
            StringRiffle[StringReplace[ToString[s], {"#" -> "a", "{" -> "|", "}" -> "|"}] <> expr2str[#]& /@ gps2, ","],
            n
        }
    ]
];


Export[
    FileNameJoin[{NotebookDirectory[], "tree4.rs"}],
    "use crate::{Maybe32,AST::{self,*}};\n" <> StringRiffle[o4 /@ {2, 3, 4}, "\n\n"],
    "Text"
]
