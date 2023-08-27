<?php

namespace Nicelocal\Mongo;

use Revolt\EventLoop;
use Revolt\EventLoop\Suspension;

final class Client {
    private static ?string $id = null;
    
    public static function register(): void {
        if (self::$id !== null) {
            return;
        }

        $f = fopen("php://fd/".\mongo_async_init(), 'r+');
        stream_set_blocking($f, false);
        self::$id = EventLoop::onReadable($f, fn () => \mongo_async_wakeup());
    }

    public static function reference(): void{
        EventLoop::reference(self::$id);
    }
    public static function unreference(): void {
        EventLoop::unreference(self::$id);
    }
}