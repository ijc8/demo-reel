global int mouseX;
global Event mouseClicked;
SinOsc foo => dac;
300 => float currentBaseFrequency;

// The sine wave's frequency is mapped to 
// the x position of the mouse...
fun void RespondToMouseMovement()
{
    while( true )
    {
        currentBaseFrequency + 0.3 * mouseX => foo.freq;
        10::ms => now;
    }
}
spork ~ RespondToMouseMovement();

//... plus some randomness!
fun void UpdateBaseFrequency()
{
    while( true )
    {
        Math.random2f( 300, 600 ) => currentBaseFrequency;
        200::ms => now;
    }
}
spork ~ UpdateBaseFrequency();

// It will stop playing when you click the mouse
// (at least 1 minute after starting)
// (you can also stop it with the "remove" button below)
5::second => now;
// 1::minute => now;
// mouseClicked => now;
