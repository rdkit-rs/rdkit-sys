#pragma once

#include "rust/cxx.h"

namespace RDKit {
    rust::String mol_to_inchi(std::shared_ptr<ROMol> mol);
    std::shared_ptr<ROMol> inchi_to_mol(const std::string &inchi);
}