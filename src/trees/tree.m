(* ::Package:: *)

o4[n_] := Block[
    {s = Slot /@ Range[n], gp1, gp2},
    gp1 = Groupings[Permutations[s, {n}], {Plus -> {2, Orderless}, Subtract -> 2, Times -> {2, Orderless}, Divide -> 2}, HoldForm];
    gp2 = DeleteDuplicatesBy[gp1, Factor@*ReleaseHold];
    StringRiffle[{
        "pub const V",
        n,
        "O4: &[fn(",
        StringRiffle[ConstantArray["Maybe32", n], ","],
        ") -> Maybe32; ",
        Length@gp2,
        "] = &[\n",
        StringRiffle[TemplateApply[StringReplace[ToString[s], {"#" -> "a", "{" -> "|", "}" -> "|"}] <> "``", {StringReplace[ToString@InputForm@#, {"#" -> "a", "HoldForm[" -> "{", "]" -> "}"}]}]& /@ gp2, ",\n"],
        "\n];"
    }, ""]
];


Export[
    FileNameJoin[{NotebookDirectory[], "tree4.rs"}],
    "use crate::trees::Maybe32;" <> StringRiffle[{o4[2], o4[3], o4[4]}, "\n\n"],
    "Text"
]
