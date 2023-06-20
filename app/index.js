import init, { World } from "./pkg/hello_wasm.js";

init().then(() => {
    var animating = true;
    var last_draw = 0;

    const marble = $('<canvas>').attr('width', 21).attr('height', 21)[0]
    const marblectx = marble.getContext('2d')
    const gradient = marblectx.createLinearGradient(10, 0, 10, 20);
    gradient.addColorStop(0, "#c8c8c8");
    gradient.addColorStop(1, "blue");
    marblectx.fillStyle = gradient;
    marblectx.ellipse(10, 10, 10, 10, 0, 0, 2*Math.PI)
    marblectx.fill()

    const ctx = document.getElementById('mycanvas').getContext("2d")

    const world = World.new(innerWidth, innerHeight)
    const draw_marble = (x, y) => ctx.drawImage(marble, x, y);

    function draw(time) {
        if (animating) {
            world.step(time - last_draw);
            ctx.clearRect(0, 0, innerWidth, innerHeight)
            world.draw(draw_marble);
        }
        last_draw = time;
        requestAnimationFrame(draw)
    }
    requestAnimationFrame(draw);

    function resize() {
        $('canvas').attr('width', innerWidth).attr('height', innerHeight);
        world.resize(innerWidth, innerHeight);
    };
    resize();

    var resizeTimeout = null;
    $(window).on('resize', function() {
        clearTimeout(resizeTimeout);
        setTimeout(resize, 250);
    }).on('click keydown', function(event) {
        animating = !animating;
    });
});

