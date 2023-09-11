#include "rust/cxx.h"
#include <GraphMol/GraphMol.h>
#include <GraphMol/ScaffoldNetwork/ScaffoldNetwork.h>

namespace RDKit {
  using ScaffoldNetworkParams = ScaffoldNetwork::ScaffoldNetworkParams;

  std::shared_ptr<ScaffoldNetworkParams> default_scaffold_network_params() {
    ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams();
    return std::shared_ptr<ScaffoldNetworkParams>(network_params);
  }

  std::shared_ptr<ScaffoldNetworkParams> new_scaffold_network_params(const rust::Vec<rust::String> bondBreakersSmarts) {
    std::vector<std::string> cc_vec;

    for (auto smarts : bondBreakersSmarts) {
      cc_vec.push_back(std::string(smarts));
    }

    ScaffoldNetworkParams *network_params = new ScaffoldNetworkParams(cc_vec);
    return std::shared_ptr<ScaffoldNetworkParams>(network_params);
  }

  set_include_generic_scaffolds(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.includeGenericScaffolds = input;
  }

  include_generic_bond_scaffolds(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.includeGenericBondScaffolds = input;
  }

  include_scaffolds_without_attachments(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.includeScaffoldsWithoutAttachments = input;
  }

  include_scaffolds_with_attachments(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.includeScaffoldsWithAttachments = input;
  }

  keep_only_first_fragment(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.keepOnlyFirstFragment = input;
  }

  prune_before_fragmenting(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.pruneBeforeFragmenting = input;
  }

  flatten_isotopes(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.flattenIsotopes = input;
  }

  flatten_chirality(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.flattenChirality = input;
  }

  flatten_keep_largest(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.flattenKeepLargest = input;
  }

  collect_mol_counts(std::shared_ptr<ScaffoldNetworkParams> params, bool input) {
    params.collectMolCounts = input;
  }

}