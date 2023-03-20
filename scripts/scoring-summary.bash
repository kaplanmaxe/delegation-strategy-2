#!/bin/bash

set -xo

script_dir=$(dirname "$0")
working_directory=${SCORING_WORKING_DIRECTORY:-"$script_dir/.."}
file_scoring_summary_rmd=${SCORING_SUMMARY_RMD:-"$script_dir/scoring-summary.Rmd"}
ui_id="$1"

if [[ -z $ui_id ]]
then
    echo "Usage: $0 <ui-id>"
    exit 1
fi

Rscript -e "rmarkdown::render('$file_scoring_summary_rmd', output_dir = '$working_directory', output_file = 'summary.md')" "$(realpath "$working_directory/scores.csv")" "$ui_id"
