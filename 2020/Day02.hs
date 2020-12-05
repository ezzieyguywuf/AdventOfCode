{-# LANGUAGE OverloadedStrings #-}
module Day02 (runMain) where

import Data.Void (Void)
import Data.Text (Text)
import Data.Either (rights, lefts)
import Control.Exception (displayException)
import qualified Data.Text as Text
import qualified Data.Text.IO as TextIO
import Control.Applicative
import Text.Megaparsec (Parsec, parse, eof)
import Text.Megaparsec.Char (string, space1, char, digitChar, letterChar) 
import qualified Text.Megaparsec.Char.Lexer as Lexer


runMain :: IO ()
runMain = do
    putStrLn "We're at dai too!"
    rawLines <- Text.lines <$> TextIO.readFile "2020/day02_input.txt"
    let n = sum . rights . fmap (runParser parseLine) $ rawLines
    mapM_ TextIO.putStrLn (lefts (fmap (runParser parseLine) rawLines))
    putStrLn $ "The number of correct passwords is: " <> show n
    let n' = sum . rights . fmap (runParser parseLine') $ rawLines
    putStrLn $ "The number of correct-correct passwords is: " <> show n'

runParser :: Parser Bool -> Text -> Either Text Int
runParser parser inline =
    case parse (parser <* eof) "" inline of
        Left err -> Left . Text.pack . displayException $ err
        Right val -> Right $ if val then 1 else 0

-- 17-19 p: pwpzpfbrcpppjppbmppp
-- 10-11 b: bbbbbbbbbbbj

data PwdPolicy = PwdPolicy Int Int Text deriving (Show)

parseLine :: Parser Bool
parseLine = do
    (PwdPolicy nMin nMax nLetter) <- lexeme parsePolicy
    pwd <- parsePwd
    let nChar = Text.count nLetter pwd
    pure (nChar >= nMin && nChar <= nMax)

parseLine' :: Parser Bool
parseLine' = do
    (PwdPolicy nMin nMax nLetter) <- lexeme parsePolicy
    pwd <- parsePwd
    let lChar = getVal pwd nMin
        rChar = getVal pwd nMax
        check = Text.count nLetter (lChar <> rChar)
    pure (check == 1)

getVal :: Text -> Int -> Text
getVal val ind = Text.singleton (Text.index val (ind - 1))

parsePwd :: Parser Text
parsePwd = Text.pack <$> some letterChar

parsePolicy :: Parser PwdPolicy
parsePolicy = do
    (nMin, nMax) <- lexeme parseMinMax
    nLetter <- Text.singleton <$> letterChar
    char ':'
    pure (PwdPolicy nMin nMax nLetter)

parseMinMax :: Parser (Int, Int)
parseMinMax = do
    nMin <- parseInt
    char '-'
    nMax <- parseInt
    pure (nMin, nMax)

parseInt :: Parser Int
parseInt = read <$> some digitChar

-- | Our parser
type Parser = Parsec Void Text

spaceConsumer :: Parser ()
spaceConsumer = Lexer.space space1 empty empty

lexeme :: Parser a -> Parser a
lexeme = Lexer.lexeme spaceConsumer
