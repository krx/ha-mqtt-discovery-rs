#!/usr/bin/env bash

function extract_configuration_structure() {
  cat $1 | grep -A 1000 '{% configuration %}' | grep -B 1000 '{% endconfiguration %}' | tail -n +2 | head -n -1
}

for integration_doc in $(ls -1 $HOME_ASSISTANT_DOCS/source/_integrations/*.mqtt.markdown) ; do
  name=$(basename $integration_doc | sed 's/\.markdown$//g')
  extract_configuration_structure $integration_doc > specs/$name.yml
done
