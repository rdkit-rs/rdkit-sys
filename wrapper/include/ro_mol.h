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
}