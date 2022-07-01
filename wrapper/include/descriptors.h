#pragma once

#include "rust/cxx.h"
#include <GraphMol/Descriptors/Property.h>
#include <GraphMol/MolOps.h>
#include <GraphMol/Descriptors/MolDescriptors.h>

namespace RDKit {
    using Descriptors::Properties;

    std::shared_ptr<Properties> new_properties();
    std::unique_ptr<std::vector<std::string>> get_property_names(std::shared_ptr<Properties> props);
    std::unique_ptr<std::vector<double>> compute_properties(std::shared_ptr<Properties> props, std::shared_ptr<ROMol> mol);
    rust::String mol_formula(std::shared_ptr<ROMol> mol);
    rust::u16 symmetrize_SSSR(std::shared_ptr<ROMol> mol);
    rust::u32 mol_exact_MW(std::shared_ptr<ROMol> mol);
}