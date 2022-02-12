function docker;
    set DOCKER (which docker);
    if test "$argv[1]" = "run";
        bt args -d | xargs -I{} $DOCKER run {} $argv[2..];
    else;
        $DOCKER $argv[1..];
    end;
end;

function pack;
    set PACK (which pack);
    if test "$argv[1]" = "build";
        bt args -p | xargs $PACK build $argv[2..];
    else;
        $PACK $argv[1..];
    end;
end;