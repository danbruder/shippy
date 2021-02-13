module Pages.Top exposing (Model, Msg, Params, page)

import Css exposing (hover)
import Css.Global
import Html.Styled exposing (..)
import Html.Styled.Attributes exposing (css, href, src)
import Spa.Document exposing (Document)
import Spa.Page as Page exposing (Page)
import Spa.Url exposing (Url)
import Tailwind.Breakpoints exposing (..)
import Tailwind.Utilities exposing (..)


type alias Params =
    ()


type alias Model =
    Url Params


type alias Msg =
    Never


page : Page Params Model Msg
page =
    Page.static
        { view = view
        }



-- VIEW


view : Url Params -> Document Msg
view { params } =
    { title = "Homepage"
    , body =
        [ body
        ]
    }


body =
    [ 1, 2, 3 ]
        |> List.map viewApp
        |> div []
        |> wrapper


wrapper innerContent =
    div []
        [ div [ css [ bg_gray_800, pb_32 ] ]
            [ div [ css [ max_w_7xl, mx_auto, pt_8, sm [ px_6 ], lg [ px_8 ] ] ]
                [ header [ css [ py_10 ] ]
                    [ div [ css [ max_w_7xl, mx_auto, px_4, sm [ px_6 ], lg [ px_8 ] ] ]
                        [ h1 [ css [ text_3xl, font_bold, text_white ] ]
                            [ text "Applications"
                            ]
                        ]
                    ]
                ]
            ]
        , div [ css [ neg_mt_32 ] ]
            [ div [ css [ max_w_7xl, mx_auto, pb_12, sm [ px_6 ], lg [ px_8 ] ] ]
                [ div [ css [ bg_white, rounded_lg, shadow, px_5, py_6, sm [ px_6 ] ] ]
                    [ div [ css [ border_4, border_dashed, border_gray_200, rounded_lg, h_96 ] ]
                        [ innerContent ]
                    ]
                ]
            ]
        ]


viewApp _ =
    a [ href "#", css [ block, hover [ bg_gray_50 ] ] ]
        [ div [ css [ flex, items_center, px_4, py_4, sm [ px_6 ] ] ]
            [ div [ css [ min_w_0, flex_1, sm [ flex, items_center, justify_between ] ] ]
                [ div []
                    [ div [ css [ flex, text_sm, text_green_600, Tailwind.Utilities.truncate ] ]
                        [ p [ css [ text_lg, font_bold ] ] [ text "API Server" ]
                        ]
                    , div [ css [ mt_2, flex ] ]
                        [ div [ css [ flex, items_center, text_sm, text_gray_500 ] ]
                            [ p [] [ text "Last deployed on Feb 2, 2020" ]
                            ]
                        ]
                    ]
                , div
                    [ css
                        [ ml_5
                        , flex_shrink_0
                        , text_gray_400
                        , font_bold
                        ]
                    ]
                    [ text ">" ]
                ]
            ]
        ]
