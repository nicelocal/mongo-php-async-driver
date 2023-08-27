<?php
require __DIR__.'/../vendor/autoload.php';

$client = new MongoDB\Client("mongodb://localhost:27017");
$collection = $client->demo->beers;

if ($collection->count()) {
    return;
}

for ($x = 0; $x < 10000; $x++) {
    $result = $collection->insertOne(['name' => 'Hinterland', 'brewery' => "BrewDog $x"]);
}