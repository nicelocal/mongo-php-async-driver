# Async MongoDB PHP extension

This PHP extension offers an async driver for MongoDB, based on the official Rust mongodb library.  

It integrates the [PHP revolt event loop](https://revolt.run) and rust's tokio event loop, offering a fully asynchronous fiber-based API for mongodb.

Currently a work-in-progress, more methods will be added soon!

Written by Daniil Gentili ([@danog](https://github.com/danog)), powered by [danog/php-tokio](https://github.com/danog/php-tokio), [nicelocal/ext-php-rs](https://github.com/Nicelocal/ext-php-rs/) and the official MongoDB Rust client.

## Example

```php
<?php

require 'vendor/autoload.php';

use Nicelocal\Mongo\Client;
use Revolt\EventLoop;

use function Amp\async;
use function Amp\Future\await as await;

Client::register();

function dump(string $database, string $collection, array $find) {
    $collection = \MongoClient::withUriStr("mongodb://localhost:27017")
        ->database($database)
        ->collection($collection);
    $cnt = 0;
    foreach ($collection->find($find) as $k => $v) {
        var_dump($k, $v);
        if ($cnt++ === 3) break;
    }
    var_dump("done");
}

$future1 = async(dump(...), 'nicelocal', 'coll1', ['field' => ['$ne' => 'test']]);
$future2 = async(dump(...), 'nicelocal', 'coll2', ['field' => ['$ne' => 'test']]);

[$res1, $res2] = await([$future1, $future2]);
```

Usage:

```
cargo build && php -d extension=target/debug/libmongo_php_async_driver.so test.php
```