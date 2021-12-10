bus_config("MBANDEDWG", "aux 0-1 out")
bus_config("FREEVERB", "aux 0-1 in", "out 0-1")

/* grab the current unix timestamp from the command line argument */

curDate = 1639103946
printf(" %f is curDate\n", curDate)
startDate = 1136133673
endDate = 2998053673
percent = (curDate - startDate) / (endDate - startDate)
daysSinceBeginning = (curDate - startDate) / 86400

/* set up some basic constants */

baseFreq = 90
numPartials = 19
totalDurInSecs = 240
/* as time passes, use more and more elements of thue morse */
sequenceLength = percent * (512 - 32) + 32
sequenceStartIndex = 1
thuemorse = { }

/* compute the thue morse terms we want to use for this run */

for (i=0; i<sequenceLength; i=i+1) {
        n = sequenceStartIndex + i
        temp = n
        result = 0
        while (temp > 0) {
                result = result + temp %2
                temp = trunc(temp / 2)
        }
        thuemorse[i] = result
}

/* make the music one partial at a time */

for (partial=1; partial < numPartials; partial=partial+1) {
        printf("partial: %d\n", partial)
        /* higher partials make faster repeating rhythms */
        skip = (numPartials - partial) * 0.125
        for (st=0; st<totalDurInSecs; st=st+skip) {
                /* get current array index based on how far we are into the piece */
                positionIndex = trunc((st / totalDurInSecs) * len(thuemorse))
                /* layer one starts with highest partial at beginning and adds in descending */
                if ((thuemorse[positionIndex] >= (numPartials - partial))) {
                        /* amplitude of this layer decreases over time
                         * and lower partials are quieter */
                        amp = pow(10, ((50 + 15 - pow(15 * st / totalDurInSecs, 0.5)) * (0.65 + 0.35 * partial / numPartials))/20)
                        /* small random variation in amplitude of each note */
                        amp = amp * irand(0.90, 1.1)
                        /* occasional accents added in */
                        if (irand(0, 1) < 0.25 * pow(((numPartials - partial) / numPartials), 0.5)) { amp = amp * 3 }
                        /* panning based on partial number */
                        pan = 0.25 + 0.5 - 0.5 * partial / numPartials
                        /* finally, play the note! */
                        MBANDEDWG(st, skip, amp, baseFreq*partial, 0, 0, 1, 0, 0, 0.99, 0, pan)
                }
                /* layer two starts with lowest partial at beginning and adds in ascending */
                if (thuemorse[positionIndex] >= partial) {
                        /* amplitude of this layer increases over time
                         * and higher partials are quieter */
                        amp = pow(10, ((65 + 15 * st / totalDurInSecs) * (0.65 + 0.35 * (numPartials - partial) / numPartials))/20)
                        /* small random variation in amplitude of each note */
                        amp = amp * irand(0.95, 1.05)
                        /* occasional accents added in */
                        if (irand(0, 1) < 0.25 * pow(((numPartials - partial) / numPartials), 0.5)) { amp = amp * 3 }
                        /* panning based on partial number */
                        pan = 0.25 + 0.5 - 0.5 * partial / numPartials
                        /* finally, play the note! */
                        MBANDEDWG(st, skip, amp, baseFreq*partial, 0, 0, 1, 0, 0, 0.99, 0, pan)
                }
        }
}

/* add in some reverb that increases as the piece progresses */
dry = maketable("line", "nonorm", 100, 0,95, totalDurInSecs,65)
wet = maketable("line", "nonorm", 100, 0,5, totalDurInSecs,35)
FREEVERB(0, 0, totalDurInSecs + 5, 1.25, 0.9, 0.001, 4.0, 71, dry, wet, 25)
