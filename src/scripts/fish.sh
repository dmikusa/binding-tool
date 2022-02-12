function docker
    set DOCKER (which docker)
    if test "$argv[1]" = "run"
        $DOCKER run $argv[2..] ###REPLACE###
    else
        $DOCKER $argv[1..]
    end
end

function pack
    set PACK (which pack)
    if test "$argv[1]" = "build"
        $PACK build $argv[2..] ###REPLACE###
    else
        $PACK $argv[1..]
    end
end