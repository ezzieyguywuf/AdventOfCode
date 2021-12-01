module Day03 (runMain) where

import Data.List (uncons)
import Data.Text (Text)
import qualified Data.Text as Text
import qualified Data.Text.IO as TextIO
import Control.Monad.State (State, get, put, execState)

runMain :: IO ()
runMain = do
    putStrLn "Can we do 3 in one day??!"
    rawLines <- take 4 . Text.lines <$> TextIO.readFile "2020/day03_input.txt" 

    mapM_ TextIO.putStrLn rawLines
    moveTobaggon rawLines

type MyState = State Tobaggon

data Tobaggon =
    Tobaggon
        { getCol   :: Int
        , getRow   :: Int
        , getTrees :: Int
        } deriving (Show)


-- Blahhhh State!!!!
-- We can do this
moveTobaggon :: [Text] -> MyState Bool
moveTobaggon theMap = do
    (Tobaggon col row trees) <- get
    let newCol = if col' > rowLen then col' - rowLen  else col'
        col' = col + 3
        row' = row + 1
        rowLen = Text.length (head theMap)
        curRow = theMap !! row'
    if row' > length theMap
        then pure False
        else do
            let trees' = Text.index curRow newCol == '#'
            put (Tobaggon newCol row' trees')
            pure True
