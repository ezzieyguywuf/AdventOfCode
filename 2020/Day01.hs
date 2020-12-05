module Day1 (day1Main) where

day1Main :: IO ()
day1Main = do
    -- Say hi
    putStrLn "Oh hai!"

    -- Get data
    rawData <- fmap read . lines <$> readFile "2020/day1_input.txt" :: IO [Int]
    putStrLn $ "The code is: " <> show (findPair 2020 rawData)
    putStrLn $ "The second code is: " <> show (findTriple rawData)
    pure ()

findPair :: Int -> [Int] -> Maybe Int
findPair target (x:xs) =
    case checkVal target x xs of
        Nothing -> findPair target xs
        Just val -> Just val
findPair _ _ = Nothing

findTriple :: [Int] -> Int
findTriple (x:xs) =
    let target = 2020 - x
    in case findPair target xs of
           Nothing -> findTriple xs
           Just val -> val * x
findTriple _ = 0

-- Find 2 vals that sum to the given value
checkVal :: Int -> Int -> [Int] -> Maybe Int
checkVal target x (y:ys) = if x + y == target
                        then Just (x * y)
                        else checkVal target x ys
checkVal _ _ _ = Nothing
