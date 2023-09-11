#pragma once

#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
  using ScaffoldNetworkParams = ScaffoldNetwork::ScaffoldNetworkParams;

  std::shared_ptr<ScaffoldNetworkParams> default_scaffold_network_params();
  std::shared_ptr<ScaffoldNetworkParams> new_scaffold_network_params(const rust::Vec<rust::String> bondBreakersSmarts);

  set_include_generic_scaffolds(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  include_genericbond_scaffolds(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  include_scaffolds_without_attachments(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  include_scaffolds_with_attachments(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  keep_only_first_fragment(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  prune_before_fragmenting(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  flatten_isotopes(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  flatten_chirality(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  flatten_keep_largest(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
  collect_mol_counts(std::shared_ptr<ScaffoldNetworkParams> params, bool input);
}