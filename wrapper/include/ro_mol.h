#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/SmilesParse/SmilesParse.h>
#include <GraphMol/SmilesParse/SmilesWrite.h>
#include <DataStructs/ExplicitBitVect.h>
#include <GraphMol/Fingerprints/Fingerprints.h>
#include <GraphMol/MolStandardize/Tautomer.h>

namespace RDKit {
    std::shared_ptr<ROMol> copy_mol(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ROMol> smiles_to_mol(const std::string &smiles);
    rust::String mol_to_smiles(std::shared_ptr<ROMol> mol);

    std::shared_ptr<ROMol> smiles_to_mol_with_params(const std::string &smiles, std::shared_ptr<SmilesParserParams> params);
    std::shared_ptr<SmilesParserParams> new_smiles_parser_params();
    void smiles_parser_params_set_sanitize(std::shared_ptr<SmilesParserParams> params, bool sanitize);

    std::unique_ptr<std::vector<std::string>> detect_chemistry_problems(std::shared_ptr<ROMol> mol);

    unsigned int get_num_atoms(std::shared_ptr<ROMol> mol, bool only_explicit);

    std::shared_ptr<Atom> get_atom_with_idx(std::shared_ptr<ROMol> mol, unsigned int idx);

//    pub fn get_symbol(atom: SharedPtr<Atom>) -> CxxString;
    rust::String get_symbol(std::shared_ptr<Atom> atom);

    void update_property_cache(std::shared_ptr<ROMol> mol, bool strict);
}
