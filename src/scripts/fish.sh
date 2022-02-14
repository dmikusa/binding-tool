function docker;
    set DOCKER (which docker);
    if test "$argv[1]" = "run";
        $DOCKER run (bt args -d | string split -n ' ') $argv[2..];
    else;
        $DOCKER $argv[1..];
    end;
end;

function pack;
    set PACK (which pack);
    if test "$argv[1]" = "build";
        $PACK build $argv[2..] (bt args -p | string split -n ' ');
    else;
        $PACK $argv[1..];
    end;
end;