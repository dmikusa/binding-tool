function docker {
    DOCKER=$(which docker)

    if [ "$1" == "run" ]; then
        shift
        $DOCKER run $(bt args -d) "$@"
    else
        $DOCKER "$@"
    fi
}

function pack {
    PACK=$(which pack)

    if [ "$1" == "build" ]; then
        shift
        $PACK build $(bt args -p) "$@"
    else
        $PACK "$@"
    fi
}