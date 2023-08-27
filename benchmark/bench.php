<?php

use Nicelocal\Mongo\Client;

require __DIR__.'/../vendor/autoload.php';

Client::register();

$collection = (new MongoDB\Client("mongodb://localhost:27017"))->demo->beers;

$collectionAsync = \MongoClient::withUriStr("mongodb://localhost:27017")
    ->database('demo')
    ->collection('beers');

function warmup($collection): void
{
    $x = 100;
    foreach ($collection->find([]) as $elem) {
        if (!$x--) break;
    }
}

function bench($collection): float
{
    $t = microtime(true);
    foreach ($collection->find([]) as $elem) {
    }
    return microtime(true) - $t;
}

warmup($collection);
warmup($collectionAsync);

var_dump("Async: ".bench($collectionAsync));
var_dump("Sync: ".bench($collection));
