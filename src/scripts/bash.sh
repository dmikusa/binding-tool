function docker {
    DOCKER=$(which docker)

    if [ "$1" == "run" ]; then
        shift
        $DOCKER run "$@"  ###REPLACE###
    else
        $DOCKER "$@"
    fi
}

function pack {
    PACK=$(which pack)

    if [ "$1" == "build" ]; then
        shift
        $PACK build "$@"  ###REPLACE###
    else
        $PACK "$@"
    fi
}