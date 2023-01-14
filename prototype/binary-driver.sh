#!/bin/bash
# https://www.git-scm.com/docs/gitattributes#_defining_a_custom_merge_driver

baseFile=$1
ourFile=$2
theirFile=$3
targetFile=$4

# targetName=$(basename -- "${targetFile}")
targetName="${targetFile%.*}"
targetExt="${targetFile##*.}"

isConflict=false
function conflict_check() {
  local our=$1
  local compare=$2
  local compareType=$3
  if [[ "$(cmp ${our} ${compare} 2>/dev/null)" ]]; then
    isConflict=true
    if [[ "${targetExt}" == "" ]]; then
      cp "${compare}" "${targetName}.${compareType}"
    else
      cp "${compare}" "${targetName}.${compareType}.${targetExt}"
    fi
  fi
}

conflict_check "${ourFile}" "${baseFile}" "base"
conflict_check "${ourFile}" "${theirFile}" "their"

if [[ "${isConflict}" = true ]]; then
  exit 1
fi
